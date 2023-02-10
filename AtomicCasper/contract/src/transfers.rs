use alloc::string::String;
use casper_contract::contract_api::runtime::{self, call_contract};
use casper_types::{Key, ContractHash, account::{Account, AccountHash}, runtime_args, U512, U256, RuntimeArgs};

use crate::{Error, constants, utils::get_key_val};



pub fn transfer_to(secret_hash : &str){
    let saved_hash = get_key_val::<String>(constants::NAMED_KEY_HASH);
    if secret_hash != saved_hash.as_str(){
        runtime::revert(Error::HashMismatch);
    }
    let _type = get_key_val::<String>(constants::NAMED_KEY_TYPE);
    let _type = _type.as_str();
    let reciver = get_key_val::<AccountHash>(constants::NAMED_KEY_RECIVER);
    let reciver_key = Key::Account(reciver);
    let owner = get_key_val::<AccountHash>(constants::NAMED_KEY_OWNER);
    let owner_key = Key::Account(owner);
    match _type {
        "NFT" => {
            
        },
        "ERC20" => {
            let amount = get_key_val::<U512>(constants::NAMED_KEY_AMOUNT).to_u2256();
            let token = get_key_val::<ContractHash>(constants::NAMED_KEY_CONTRACT_HASH);
            let mut runtimeargs = RuntimeArgs::new();
            runtimeargs.insert("amount", amount);
            runtimeargs.insert("recipient", reciver_key);
            runtimeargs.insert("owner", owner_key);
            call_contract::<()>(
                token,
                "transfer_from",
                runtimeargs
            );
        },
        "Direct" => {

        },
        "Custom" => {

        },
        _ => {
            runtime::revert(Error::TypeNotFound);
        }
    }
}
pub fn transfer_back(){


}
pub trait U512ToU256 {
    fn to_u2256(self) -> U256;
}
impl U512ToU256 for U512{
    fn to_u2256(self) -> U256 {
        let mut result = U256::zero();
        result.0[..4].clone_from_slice(&self.0[..4]);
        result
    }
}