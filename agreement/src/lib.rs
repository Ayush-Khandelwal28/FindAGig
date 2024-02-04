#![allow(unused_variables)]

#[macro_use]
extern crate pbc_contract_codegen;
extern crate pbc_contract_common;

use pbc_contract_common::address::Address;
use pbc_contract_common::context::ContractContext;
use std::time::{SystemTime, UNIX_EPOCH};



/// The state of the Agreement, which is persisted on-chain.
#[state]
pub struct AgreementState {
    project_owner: Address,
    freelancer: Address,
    project_name: String,
    project_description: String,
    project_metrics: String,
    project_stake: u64,
    freelancer_stake: u64,
    is_agreement_signed_freelancer: bool,
    is_agreement_signed_owner: bool,
    current_time: Option<u64>,
}

#[init]
pub fn initialize(
    ctx: ContractContext,
    project_owner: Address,
    freelancer: Address,
    project_name: String,
    project_description: String,
    project_metrics: String,
    project_stake: u64,
    freelancer_stake: u64,
) -> AgreementState {
    AgreementState {
        project_owner,
        freelancer,
        project_name,
        project_description,
        project_metrics,
        project_stake,
        freelancer_stake,
        is_agreement_signed_freelancer: false,
        is_agreement_signed_owner: false,
        current_time: None,
    }
}

#[action(shortname = 0x01)]
pub fn sign_and_stake_freelancer(
    ctx: ContractContext,
    mut state: AgreementState,
) -> AgreementState {
    assert_eq!(
        ctx.sender, state.freelancer,
        "Only the freelancer can sign the agreement."
    );
    state.is_agreement_signed_freelancer = true;
    state
    // Implementation of Staking Pending
}

#[action(shortname = 0x02)]
pub fn sign_and_stake_owner(ctx: ContractContext, mut state: AgreementState) -> AgreementState {
    assert_eq!(
        ctx.sender, state.project_owner,
        "Only the project owner can sign the agreement."
    );
    state.is_agreement_signed_owner = true;
    state
    // Implementation of Staking Pending
}

#[action(shortname = 0x03)]
pub fn start_time(ctx: ContractContext, mut state: AgreementState) -> AgreementState {
    assert_eq!(
        ctx.sender, state.project_owner,
        "Only the project owner can start the project."
    );

    // Check if both the freelancer and project owner have signed the agreement
    // assert!(
    //     state.is_agreement_signed_freelancer && state.is_agreement_signed_owner,
    //     "Both freelancer and project owner must sign the agreement before starting the project."
    // );

    // Update the updated_at field with the current time
    state.current_time = Some(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs());
    // Perform other actions related to starting the project if needed

    state
}

