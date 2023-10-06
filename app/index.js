// script to interact with stellar.... 
// importing the necessary modules
import {Contract, Keypair, Networks, Server, xdr, TransactionBuilder,} from 'soroban-client';

import { Account, Operation,  } from "stellar-base";

let server = new Server("");

let keypair1 = Keypair.fromSecret();

let account = await server.getAccount(keypair1.publicKey());

const dao_id = "";
const dao_contract = new Contract(dao_id);

const dao_token_id = "";
const token_contract = new Contract(dao_token_id);

const init_token = async () => {

};

const init_dao = async () => {

}

const bigInt_to_buffer = (bn) => {

}

const bigNumber_from_bytes = (...bytes) => {

}

const get_min_prop_power = async () => {

}


await get_min_prop_power();
