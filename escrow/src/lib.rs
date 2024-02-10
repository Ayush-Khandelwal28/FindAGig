#![allow(unused_variables)]

#[macro_use]
extern crate pbc_contract_codegen;
extern crate pbc_contract_common;

use pbc_contract_common::address::Address;
use pbc_contract_common::context::ContractContext;

/// The state of the Escrow, which is persisted on-chain.
#[state]
pub struct EscrowState {
    project_owner: Address,
    freelancer: Address,
    arbiter: Address,
    project_owner_stake: u64,
    freelancer_stake: u64,
    released: bool,
    arbiter_settled: bool,
    project_submitted: bool,
    project_approved: bool,
    code_link: Option<String>,
    assets_link: Option<String>,
    comments: Option<String>,
}

#[init]
pub fn initialize(ctx: ContractContext, freelancer: Address, arbiter: Address, project_owner_stake: u64) -> EscrowState {
    EscrowState {
        project_owner: ctx.sender,
        freelancer,
        arbiter,
        project_owner_stake,
        freelancer_stake: 0,
        released: false,
        arbiter_settled: false,
        project_submitted: false,
        project_approved: false,
        code_link: None,
        assets_link: None,
        comments: None,
    }
}

#[action(shortname = 0x01)]
pub fn stake_freelancer(ctx: ContractContext, mut state: EscrowState) -> EscrowState {
    require_freelancer(ctx, &state);
    require_not_released(&state);

    // Staking a predetermined amount (e.g., 5)
    let freelancer_stake_amount = 5;
    state.freelancer_stake = freelancer_stake_amount;

    state
}

#[action(shortname = 0x05)]
pub fn submit_project(ctx: ContractContext, mut state: EscrowState, code_link: String, assets_link: Option<String>, comments: Option<String>) -> EscrowState {
    require_freelancer(ctx, &state);
    require_not_released(&state);
    require_project_not_submitted(&state);
    require_non_empty_code_link(&code_link);

    // Your logic for handling the project submission goes here.
    // Store the provided links and comments in the contract state.
    state.code_link = Some(code_link);
    state.assets_link = assets_link;
    state.comments = comments;

    state.project_submitted = true;
    state
}

#[action(shortname = 0x06)]
pub fn approve_project(ctx: ContractContext, mut state: EscrowState) -> EscrowState {
    require_project_owner(ctx, &state);
    require_project_submitted(&state);
    require_project_not_approved(&state);

    // Your logic for approving the project goes here.
    // For example, mark the project as approved in the contract state.
    state.project_approved = true;

    state
}

#[action(shortname = 0x07)]
pub fn release_funds(ctx: ContractContext, mut state: EscrowState) -> EscrowState {
    require_project_owner(ctx, &state);
    require_project_submitted(&state);
    require_project_approved(&state);
    require_not_released(&state);

    // Your logic for releasing funds goes here.
    // For example, mark the funds as released in the contract state.
    state.released = true;

    state
}

fn require_freelancer(ctx: ContractContext, state: &EscrowState) {
    assert!(
        ctx.sender == state.freelancer,
        "Only the freelancer can perform this action."
    );
}

fn require_project_owner(ctx: ContractContext, state: &EscrowState) {
    assert!(
        ctx.sender == state.project_owner,
        "Only the project owner can perform this action."
    );
}

fn require_not_released(state: &EscrowState) {
    assert!(
        !state.released,
        "Funds have already been released."
    );
}

fn require_project_not_submitted(state: &EscrowState) {
    assert!(
        !state.project_submitted,
        "Project has already been submitted."
    );
}

fn require_project_not_approved(state: &EscrowState) {
    assert!(
        !state.project_approved,
        "Project has already been approved."
    );
}

fn require_project_submitted(state: &EscrowState) {
    assert!(
        state.project_submitted,
        "Project has not been submitted yet."
    );
}

fn require_project_approved(state: &EscrowState) {
    assert!(
        state.project_approved,
        "Project has not been approved yet."
    );
}

fn require_non_empty_code_link(code_link: &str) {
    assert!(
        !code_link.is_empty(),
        "Code link cannot be blank."
    );
}
