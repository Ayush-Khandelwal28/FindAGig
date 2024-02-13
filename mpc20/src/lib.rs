#[macro_use]
extern crate pbc_contract_codegen;

use create_type_spec_derive::CreateTypeSpec;
use read_write_rpc_derive::ReadWriteRPC;
use std::ops::{Add, Sub};

use pbc_contract_common::address::Address;
use pbc_contract_common::context::ContractContext;
use pbc_contract_common::sorted_vec_map::SortedVecMap;

#[state]
pub struct TokenState {
    name: String,
    decimals: u8,
    symbol: String,
    owner: Address,
    total_supply: u128,
    balances: SortedVecMap<Address, u128>,
}

trait BalanceMap<K: Ord, V> {
    fn insert_balance(&mut self, key: K, value: V);
}

impl<V: Sub<V, Output = V> + PartialEq + Copy> BalanceMap<Address, V> for SortedVecMap<Address, V> {
    #[allow(clippy::eq_op)]
    fn insert_balance(&mut self, key: Address, value: V) {
        let zero = value - value;
        if value == zero {
            self.remove(&key);
        } else {
            self.insert(key, value);
        }
    }
}

impl TokenState {
    pub fn balance_of(&self, owner: &Address) -> u128 {
        self.balances.get(owner).copied().unwrap_or(0)
    }
}

#[init]
pub fn initialize(
    ctx: ContractContext,
    name: String,
    symbol: String,
    decimals: u8,
    total_supply: u128,
) -> TokenState {
    let mut balances = SortedVecMap::new();
    balances.insert_balance(ctx.sender, total_supply);

    TokenState {
        name,
        symbol,
        decimals,
        owner: ctx.sender,
        total_supply,
        balances,
    }
}

#[derive(ReadWriteRPC, CreateTypeSpec)]
pub struct Transfer {
    pub to: Address,
    pub amount: u128,
}

#[action(shortname = 0x01)]
pub fn transfer(
    context: ContractContext,
    state: TokenState,
    to: Address,
    amount: u128,
) -> TokenState {
    let sender = context.sender;
    core_transfer(sender, state, to, amount)
}

#[action(shortname = 0x02)]
pub fn bulk_transfer(
    context: ContractContext,
    mut state: TokenState,
    transfers: Vec<Transfer>,
) -> TokenState {
    for t in transfers {
        state = core_transfer(context.sender, state, t.to, t.amount);
    }
    state
}

#[action(shortname = 0x03)]
pub fn transfer_from(
    context: ContractContext,
    mut state: TokenState,
    from: Address,
    to: Address,
    amount: u128,
) -> TokenState {
    let caller = context.sender;
    if caller == state.owner {
        state = core_transfer(from, state, to, amount);
    } else {
        panic!("Only the contract owner can perform transfer_from operation.");
    }
    state
}

pub fn core_transfer(
    sender: Address,
    mut state: TokenState,
    to: Address,
    amount: u128,
) -> TokenState {
    let from_amount = state.balance_of(&sender);
    let o_new_from_amount = from_amount.checked_sub(amount);
    match o_new_from_amount {
        Some(new_from_amount) => {
            state.balances.insert_balance(sender, new_from_amount);
        }
        None => {
            panic!(
                "Insufficient funds for transfer: {}/{}",
                from_amount, amount
            );
        }
    }
    let to_amount = state.balance_of(&to);
    state.balances.insert_balance(to, to_amount.add(amount));
    state
}
