// #![doc = include_str!("../README.md")]
#![allow(unused_variables)]

#[macro_use]
extern crate pbc_contract_codegen;
extern crate pbc_contract_common;

use pbc_contract_common::address::Address;
use pbc_contract_common::context::ContractContext;
use pbc_contract_common::sorted_vec_map::{SortedVecMap};

/// The state of the Bidding, which is persisted on-chain.
#[state]
pub struct BiddingState {
    // signed_by: SortedVecSet<Address>,
    project_owner: Address,
    project_name: String,
    project_description: String,
    project_metrics: String,
    curr_winning_bid: u64,
    bids: SortedVecMap<Address, u64>,
    bids_comment: SortedVecMap<Address, String>,
}

#[init]
pub fn initialize(ctx: ContractContext, project_name: String, project_description: String, project_metrics: String) -> BiddingState {
    assert_ne!(
        project_name, "",
        "The name of the project cannot be empty."
    );
    assert_ne!(
        project_description, "",
        "The description of the project cannot be empty."
    );
    assert_ne!(
        project_metrics, "",
        "The metrics of the project cannot be empty."
    );
    BiddingState {
        project_name,
        project_description,
        project_metrics,
        curr_winning_bid: u64::MAX,
        bids: SortedVecMap::new(),
        bids_comment: SortedVecMap::new(),
        project_owner: ctx.sender,
    }
}

#[action(shortname = 0x01)]
pub fn bid(ctx: ContractContext, mut state: BiddingState, value: u64, comment: String) -> BiddingState {
    // state.signed_by.insert(ctx.sender);
    state.bids.insert(ctx.sender, value);
    state.bids_comment.insert(ctx.sender, comment);
    
    if state.bids.len() == 1 {
        if let Some((address, bid_value)) = state.bids.iter().next() {
            state.curr_winning_bid = *bid_value; // Note the dereference here
        }
    } else if value < state.curr_winning_bid {
        state.curr_winning_bid = value;
    }
    
    state
}

