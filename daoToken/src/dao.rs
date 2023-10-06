use core::ops::Add;

use soroban_sdk::{
    contracterror, contractimpl, contracttype, panic_with_error, symbol, unwrap::UnwrapOptimized,
    Address, Env, Vec,
}

use crate::balance::{receive_balance, spend_balance};

#[derive(Clone)]
#[contracttype]

pub struct PowerAtArgs{
    block: u32,
    ident: Address,
}

#[contracttype]
pub enum DaoDataKey {
    PChanges(Address),
    PowerAt(PowerAtArgs),
    Power(Address),
    DelegateTo(DelegateAmountArgs),
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]

pub enum DaoError {
    NotEnoughToken = 0,
    CannotDelegateNegative = 1,
    PowerCannotBeNegative = 2,
    CannotAddNegativePower = 3,
    CannotRemoveNegativePower = 4,
    IncorrectNonce = 5,
}


pub trait DaoExtensionTrait {
    // Get voting power of a Identifier
    // We explicitly use Identifier instead of Address to allow for threshold signature schemes like FROST
    fn power(env: Env, of: Address) -> i128;
    fn power_at(env: Env, of: Address, at_block: u32) -> i128;
    // delegate power `from` to `to`
    fn delegate(env: Env, from: Address, to: Address, amount: i128);
    // remove delegation
    // amount is the amount we want to remove
    // from is the person who delegated
    fn r_delegate(env: Env, from: Address, to: Address, amount: i128);
    //get amount that `from` has delegated to `to`
    fn get_d_a(env: Env, from: Address, to: Address) -> i128;
}

struct DaoExtension;
#[contractimpl]
impl DaoExtensionTrait for DaoExtension {
    fn power(env: Env, of: Address) -> i128 {
        return get_power(&env, of);
    }

    fn power_at(env: Env, of: Address, at_block: u32) -> i128 {
        get_power_at_or_before(&env, of, at_block)
    }

    fn delegate(env: Env, from: Address, to: Address, amount: i128) {
        from.require_auth();

        add_delgation(&env, from, to, amount)
    }

    fn r_delegate(env: Env, from: Address, to: Address, amount: i128) {
        from.require_auth();
        remove_delegation(&env, from, to, amount)
    }

    fn get_d_a(env: Env, from: Address, to: Address) -> i128 {
        get_delagate_amount_from_to(&env, from, to)
    }
}


