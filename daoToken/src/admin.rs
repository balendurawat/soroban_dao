// Import statements
use crate::storage_types::DataKey;
use soroban_sdk::{Address, Env};

// Function to check if an administrator is set in the contract's storage
pub fn has_administrator(e: &Env) -> bool {
    // Define the key for the administrator data in storage
    let key = DataKey::Admin;

    // Check if the key exists in the contract's storage and return true if it does, false otherwise
    e.storage().has(&key)
}

// Function to read the administrator's address from the contract's storage
fn read_administrator(e: &Env) -> Address {
    // Define the key for the administrator data in storage
    let key = DataKey::Admin;

    // Retrieve the administrator's address associated with the key and unwrap the result (may panic if the key doesn't exist)
    e.storage().get_unchecked(&key).unwrap()
}

// Function to write the administrator's address to the contract's storage
pub fn write_administrator(e: &Env, id: &Address) {
    // Define the key for the administrator data in storage
    let key = DataKey::Admin;

    // Store the provided administrator's address in the contract's storage
    e.storage().set(&key, id);
}

// Function to check if an action is authorized by the administrator
pub fn check_admin(e: &Env, admin: &Address) {
    // Compare the provided admin address with the address stored in the contract's storage
    if admin != &read_administrator(e) {
        // If they don't match, raise a panic indicating that the action is not authorized by the admin
        panic!("not authorized by admin");
    }
}
