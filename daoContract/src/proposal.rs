use soroban_sdk::{
    contracttype, panic_with_error, unwrap::UnwrapOptimized, Address, BytesN, Env, RawVal, Symbol,
    Vec,
}

use crate::{data_keys::DataKey, errors::ContractError, settings::get_min_prop_duration};


#[contracttype]
#[derive(Clone, Debug)]
pub struct ProposalVoted {
    pub voter: Address,
    pub prop_id: u32,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct ProposalInstr {
    //contract id
    pub c_id: BytesN<32>,
    pub fun_name: Symbol,
    pub args: Vec<RawVal>,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct Proposal {
    pub end_time: u64,
    // instrunctions will be executed in sequence
    pub instr: Vec<ProposalInstr>,
}

#[contracttype]
#[derive(Clone)]
pub struct VotesCount {
    pub v_for: i128,
    pub v_against: i128,
    pub v_abstain: i128,
}