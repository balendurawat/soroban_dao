use soroban_sdk::{BytesN, Env};

use crate::data_keys::DataKey;

pub mod tokenclient {
    soroban_sdk::contractimport!(file = "../");
}

pub fn get_dao_token_client(env:: &Env) -> tokenclient::Client {
    let token_id: BytesN<32> = env
    .storage()
    .get(&DataKey::DaoToken)
    .unwrap()
    .unwrap();

    tokenclient::Client::new(&env, &token_id)
}

pub fn store_dao_token(env: &Env, token_id: BytesN<32>) {
    env.storage().set(&DataKey::Daotoken, &token_id)
}