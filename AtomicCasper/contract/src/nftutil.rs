use alloc::vec::Vec;
use casper_contract::contract_api::runtime::call_contract;
use casper_types::{ContractHash, RuntimeArgs, Key, account::AccountHash};

use crate::{constants, utils::{get_key_val, get_contract_hash, get_contract_package_hash, toKey}, TokenId};

fn get_owner_of(token_id: TokenId) -> Option<Key> {
    let contract_hash = get_key_val::<ContractHash>(constants::NAMED_KEY_CONTRACT_HASH);
    let mut runtimeargs = RuntimeArgs::new();
    runtimeargs.insert("token_id", token_id);
    let owner = call_contract::<Option<Key>>(contract_hash, "owner_of", runtimeargs);
    owner
}

pub fn check_nfts_ownership(token_ids : Vec<TokenId>) -> bool {
    for token_id in token_ids {
        let owner = get_owner_of(token_id);
        if owner.is_none() || owner.unwrap() != get_contract_package_hash().to_key(){
            return false;
        }
    }
    true
}

pub fn transfer_to(reciver : Key){
    let token_ids = get_key_val::<Vec<TokenId>>(constants::NAMED_KEY_TOKEN_IDS);
    transfer_tokens(reciver, token_ids);
}
pub fn transfer_back(){
    let reciver = get_key_val::<Key>(constants::NAMED_KEY_OWNER);
    let token_ids = get_key_val::<Vec<TokenId>>(constants::NAMED_KEY_TOKEN_IDS);
    transfer_tokens(reciver, token_ids);
}
fn transfer_tokens(reciver : Key, token_ids : Vec<TokenId>){
    let contract_hash = get_key_val::<ContractHash>(constants::NAMED_KEY_CONTRACT_HASH);
    let mut runtimeargs = RuntimeArgs::new();
    runtimeargs.insert("recipient", reciver);
    runtimeargs.insert("token_ids", token_ids);
    let _ = call_contract::<()>(
        contract_hash,
        "transfer",
        runtimeargs
    );
}