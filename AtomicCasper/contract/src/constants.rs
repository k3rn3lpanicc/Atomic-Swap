use alloc::string::String;
use alloc::string::ToString;
use casper_contract::contract_api::storage;
use casper_types::{
    contracts::{NamedKeys, Parameters},
    EntryPoint, EntryPoints,
};

pub const NAMED_KEY_TOKEN_IDS: &str = "token_ids";
pub const NAMED_KEY_HASH: &str = "hash";
pub const NAMED_KEY_HASH_TYPE: &str = "hash_type";
pub const NAMED_KEY_SECRET: &str = "secret";
pub const NAMED_KEY_TYPE: &str = "type";
pub const NAMED_KEY_OWNER: &str = "owner";
pub const NAMED_KEY_RECIVER: &str = "reciver";
pub const NAMED_KEY_CONTRACT_HASH: &str = "contract_hash";
pub const NAMED_KEY_OWN_CONTRACT_PACKAGE_HASH: &str = "own_contract_package_hash";
pub const NAMED_KEY_AMOUNT: &str = "amount";
pub const NAMED_KEY_START_TIME: &str = "start_time";
pub const NAMED_KEY_END_TIME: &str = "end_time";
pub const NAMED_KEY_PURSE: &str = "purse";
pub const NAMED_KEY_OWN_CONTRACT_HASH : &str = "own_contract_hash";

pub const ARG_SECRET: &str = "secret";
pub const ARG_CONTRACT_HASH: &str = "contract_hash";
pub const ARG_HASH: &str = "hash";
pub const ARG_HASH_TYPE: &str = "hash_type";
pub const ARG_TIMEOUT: &str = "timeout";
pub const ARG_TYPE: &str = "type";
pub const ARG_AMOUNT: &str = "amount";
pub const ARG_PURSE: &str = "purse";
pub const ARG_RECEIVER: &str = "receiver";
pub const ARG_TOKEN_IDS: &str = "token_ids";


pub fn get_entrypoints() -> EntryPoints {
    let mut result = EntryPoints::new();
    result.add_entry_point(EntryPoint::new(
        "init",
        Parameters::new(),
        casper_types::CLType::Unit,
        casper_types::EntryPointAccess::Public,
        casper_types::EntryPointType::Contract,
    ));
    result
}

pub fn get_named_keys() -> alloc::collections::BTreeMap<alloc::string::String, casper_types::Key> {
    let mut named_keys: NamedKeys = NamedKeys::new();
    named_keys.insert(
        NAMED_KEY_HASH.to_string(),
        storage::new_uref(String::from("Empty")).into(),
    );
    named_keys
}
