use crate::admin::{check_admin, has_administrator, write_administrator};
use crate::allowance::{read_allowance, spend_allowance, write_allowance};
use crate::balance::{is_authorized, write_authorization};
use crate::balance::{read_balance, receive_balance, spend_balance};
use crate::event;
use crate::metadata::{
    read_decimal, read_name, read_symbol, write_decimal, write_name, write_symbol,
};
use soroban_sdk::{contractimpl, Address, Bytes, Env};


// Define a trait named TokenTrait that outlines the common functionality
// for managing tokens within a Soroban blockchain contract.
pub trait TokenTrait {
    // Initialize the token contract with initial parameters.
    fn initialize(e: Env, admin: Address, decimal: u32, name: Bytes, symbol: Bytes);

    // Get the allowance amount approved by an owner for a spender to spend.
    fn allowance(e: Env, owner: Address, spender: Address) -> i128;

    // Increase the allowance granted by an owner to a spender.
    fn increase_allowance(e: Env, owner: Address, spender: Address, amount: i128);

    // Decrease the allowance granted by an owner to a spender.
    fn decrease_allowance(e: Env, owner: Address, spender: Address, amount: i128);

    // Get the token balance of an account.
    fn balance(e: Env, account: Address) -> i128;

    // Get the spendable token balance of an account (equivalent to the balance in this example).
    fn spendable(e: Env, account: Address) -> i128;

    // Check if an account is authorized to perform certain actions.
    fn is_authorized(e: Env, account: Address) -> bool;

    // Transfer tokens from one account to another.
    fn transfer(e: Env, from: Address, to: Address, amount: i128);

    // Transfer tokens from one account to another on behalf of a spender.
    fn transfer_from(e: Env, spender: Address, from: Address, to: Address, amount: i128);

    // Burn (reduce) tokens from an account.
    fn burn(e: Env, from: Address, amount: i128);

    // Burn tokens from an account on behalf of a spender.
    fn burn_from(e: Env, spender: Address, from: Address, amount: i128);

    // Clawback (confiscate) tokens from an account by an admin.
    fn clawback(e: Env, admin: Address, from: Address, amount: i128);

    // Set or unset authorization for an account to perform actions.
    fn set_authorization(e: Env, admin: Address, account: Address, authorize: bool);

    // Mint (create) new tokens and assign them to an account.
    fn mint_tokens(e: Env, admin: Address, recipient: Address, amount: i128);

    // Change the administrator of the token contract.
    fn change_administrator(e: Env, admin: Address, new_admin: Address);

    // Functions to retrieve token metadata.
    fn get_decimals(e: Env) -> u32;
    fn get_name(e: Env) -> Bytes;
    fn get_symbol(e: Env) -> Bytes;
}


// Define a function named `check_nonnegative_amount` that checks if an `i128` value is non-negative.
fn check_nonnegative_amount(amount: i128) {
    // Check if the `amount` is less than 0 (negative).
    if amount < 0 {
        // If it's negative, generate a panic with an error message.
        // The `{}` is a placeholder for the value of `amount`.
        panic!("Negative amount is not allowed: {}", amount);
    }
}


pub struct Token;

#[contractimpl]
impl TokenTrait for Token{
            
}