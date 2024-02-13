#[macro_use]
extern crate pbc_contract_codegen;
extern crate pbc_contract_common;

use create_type_spec_derive::CreateTypeSpec;
use pbc_contract_common::address::Address;
use pbc_contract_common::context::ContractContext;
use pbc_contract_common::events::EventGroup;
use pbc_contract_common::zk::{CalculationStatus, SecretVarId, ZkInputDef, ZkState, ZkStateChange};
use pbc_zk::Sbi32;
use read_write_state_derive::ReadWriteState;

mod zk_compute;

#[derive(ReadWriteState, Debug)]
#[repr(C)]
struct SecretVarMetadata {
    variable_type: SecretVarType,
}

#[derive(ReadWriteState, Debug, PartialEq)]
#[repr(u8)]
enum SecretVarType {
    Vote = 1,
    CountedYesVotes = 2,
}

#[derive(ReadWriteState, CreateTypeSpec, Clone)]
struct VoteResult {
    votes_for: u32,
    votes_against: u32,
    passed: bool,
}

/// This contract's state
#[state]
struct ContractState {
    /// Address that deployed the contract
    owner: Address,
    /// When the voting stops; at this point all inputs must have been made, and vote counting can
    /// now begin.
    /// Represented as milliseconds since the epoch.
    deadline_voting_time: i64,
    /// A tally that holds the number of votes for, the number of votes against,
    /// and a bool indicating whether the vote passed. It is initialized as None and is
    /// eventually updated to Some(VoteResult) after start_vote_counting is called
    vote_result: Option<VoteResult>,
    winning_bid_amount: u64,
    amount_to_be_paid: u64,
}

/// Initializes contract
///
/// # Arguments
/// * `voting_duration_ms` number of milliseconds from contract initialization where voting is
/// open
#[init(zk = true)]
fn initialize(
    ctx: ContractContext,
    _zk_state: ZkState<SecretVarMetadata>,
    voting_duration_ms: u32,
    winnig_bid: u64,
) -> ContractState {
    let deadline_voting_time = ctx.block_production_time + (voting_duration_ms as i64);
    ContractState {
        owner: ctx.sender,
        deadline_voting_time,
        vote_result: None,
        winning_bid_amount: winnig_bid,
        amount_to_be_paid: 0,
    }
}

/// Adds another vote.
#[zk_on_secret_input(shortname = 0x40)]
fn add_vote(
    context: ContractContext,
    state: ContractState,
    zk_state: ZkState<SecretVarMetadata>,
) -> (
    ContractState,
    Vec<EventGroup>,
    ZkInputDef<SecretVarMetadata, Sbi32>,
) {
    assert!(
        context.block_production_time < state.deadline_voting_time,
        "Not allowed to vote after the deadline at {} ms UTC, current time is {} ms UTC",
        state.deadline_voting_time,
        context.block_production_time,
    );
    assert!(
        zk_state
            .secret_variables
            .iter()
            .chain(zk_state.pending_inputs.iter())
            .all(|(_, v)| v.owner != context.sender),
        "Each voter is only allowed to send one vote variable. Sender: {:?}",
        context.sender
    );
    let input_def = ZkInputDef::with_metadata(SecretVarMetadata {
        variable_type: SecretVarType::Vote,
    });
    (state, vec![], input_def)
}

/// Allows anybody to start the computation of the vote.
///
/// The vote computation is automatic beyond this call, involving several steps, as described in the module documentation.
///
/// NOTE: This ignores any pending inputs
#[action(shortname = 0x01, zk = true)]
fn start_vote_counting(
    context: ContractContext,
    state: ContractState,
    zk_state: ZkState<SecretVarMetadata>,
) -> (ContractState, Vec<EventGroup>, Vec<ZkStateChange>) {
    assert!(
        context.block_production_time >= state.deadline_voting_time,
        "Vote counting cannot start before specified starting time {} ms UTC, current time is {} ms UTC",
        state.deadline_voting_time,
        context.block_production_time,
    );
    assert_eq!(
        zk_state.calculation_state,
        CalculationStatus::Waiting,
        "Vote counting must start from Waiting state, but was {:?}",
        zk_state.calculation_state,
    );

    (
        state,
        vec![],
        vec![zk_compute::count_yes_votes_start(&SecretVarMetadata {
            variable_type: SecretVarType::CountedYesVotes,
        })],
    )
}

/// Automatically called when the computation is completed
///
/// The only thing we do is to instantly open/declassify the output variables.
#[zk_on_compute_complete]
fn counting_complete(
    _context: ContractContext,
    state: ContractState,
    _zk_state: ZkState<SecretVarMetadata>,
    output_variables: Vec<SecretVarId>,
) -> (ContractState, Vec<EventGroup>, Vec<ZkStateChange>) {
    (
        state,
        vec![],
        vec![ZkStateChange::OpenVariables {
            variables: output_variables,
        }],
    )
}

/// Automatically called when a variable is opened/declassified.
///
/// We can now read the for and against variables, and compute the result
#[zk_on_variables_opened]
fn open_sum_variable(
    _context: ContractContext,
    mut state: ContractState,
    zk_state: ZkState<SecretVarMetadata>,
    opened_variables: Vec<SecretVarId>,
) -> (ContractState, Vec<EventGroup>, Vec<ZkStateChange>) {
    assert_eq!(
        opened_variables.len(),
        1,
        "Unexpected number of output variables"
    );
    let votes_for = read_variable_u32_le(&zk_state, opened_variables.get(0));
    let total_votes = zk_state
        .secret_variables
        .iter()
        .filter(|(_, x)| x.metadata.variable_type == SecretVarType::Vote)
        .count();
    let votes_against = (total_votes as u32) - votes_for;

    let vote_result = determine_result(votes_for, votes_against);
    state.vote_result = Some(vote_result);
    state.amount_to_be_paid =
        calculate_amount_to_be_paid(votes_for, votes_against, state.winning_bid_amount);

    (state, vec![], vec![])
}

/// Reads a variable's data as an u32.
fn read_variable_u32_le(
    zk_state: &ZkState<SecretVarMetadata>,
    sum_variable_id: Option<&SecretVarId>,
) -> u32 {
    let sum_variable_id = *sum_variable_id.unwrap();
    let sum_variable = zk_state.get_variable(sum_variable_id).unwrap();
    let mut buffer = [0u8; 4];
    buffer.copy_from_slice(sum_variable.data.as_ref().unwrap().as_slice());
    <u32>::from_le_bytes(buffer)
}

/// Determines the result of the vote via standard majority decision on inputs the number of votes
/// for and against.
fn determine_result(votes_for: u32, votes_against: u32) -> VoteResult {
    let passed = votes_against < votes_for;
    VoteResult {
        votes_for,
        votes_against,
        passed,
    }
}

fn calculate_amount_to_be_paid(yes_votes: u32, no_votes: u32, winning_bid_amount: u64) -> u64 {

    if yes_votes == 0 {
        return 0;
    }

    let total_votes = yes_votes + no_votes;
    if no_votes == 0 || yes_votes / 2 >= no_votes {
        return winning_bid_amount;
    } else if yes_votes > no_votes {
        return ((yes_votes as u64 * 100) / total_votes as u64 * winning_bid_amount) / 100;
    } else if yes_votes == no_votes {
        return (35 * winning_bid_amount) / 100;
    } else if no_votes > yes_votes && no_votes/2 < yes_votes {
        return (10 * winning_bid_amount) / 100;
    } else {
        return (5 * winning_bid_amount) / 100;
    }
}
