
use alloc::string::String;
use casper_contract::contract_api::storage;
use casper_types::{EntryPoint, EntryPoints, contracts::{Parameters, NamedKeys}, Parameter, Group, AccessRights};
use alloc::{string::ToString};

// pub const RUNTIME_ARG_METADATA : &str = "metadata";
// pub const RUNTIME_ARG_RECIPIENT : &str = "recipient";
// pub const RUNTIME_ARG_HOLDER_ID : &str = "holder_id";
// pub const RUNTIME_ARG_SPENDER : &str = "publisher-account";
// pub const RUNTIME_ARG_APPROVED_ID : &str = "approved_id";
// pub const RUNTIME_ARG_TOKEN_ID : &str = "token_id";
// pub const RUNTIME_ARG_COMISSION : &str = "comission";
// pub const RUNTIME_ARG_PRODUCER_ACCOUNT_HASH : &str = "producer-account";
// pub const RUNTIME_ARG_REQUEST_ID : &str = "request_id";

pub const NAMED_KEY_HASH : &str = "hash";
pub const NAMED_KEY_HASH_TYPE : &str = "hash_type";
pub const NAMED_KEY_SECRET : &str = "secret";
pub const NAMED_KEY_TYPE : &str = "type";
pub const NAMED_KEY_OWNER : &str = "owner";
pub const NAMED_KEY_RECIVER : &str = "reciver";
pub const NAMED_KEY_CONTRACT_HASH : &str = "contract_hash";
pub const NAMED_KEY_AMOUNT : &str = "amount";
pub const NAMED_KEY_START_TIME : &str = "start_time";
pub const NAMED_KEY_END_TIME : &str = "end_time";
pub const NAMED_KEY_PURSE: &str = "purse";

pub const ARG_SECRET: &str = "secret";
pub const ARG_CONTRACT_HASH: &str = "contract_hash";
pub const ARG_HASH : &str = "hash";
pub const ARG_HASH_TYPE : &str = "hash_type";
pub const ARG_TIMEOUT: &str = "timeout";
pub const ARG_TYPE: &str = "type";
pub const ARG_AMOUNT: &str = "amount";
pub const ARG_PURSE: &str = "purse";

pub fn get_entrypoints() -> EntryPoints{
    let mut result =EntryPoints::new();
    result.add_entry_point(EntryPoint::new(
        "init",
        Parameters::new(),
        casper_types::CLType::Unit,
        casper_types::EntryPointAccess::Public,
        casper_types::EntryPointType::Contract
    ));
    result
}

pub fn get_named_keys() -> alloc::collections::BTreeMap<alloc::string::String, casper_types::Key>{
    let mut named_keys : NamedKeys = NamedKeys::new();
    named_keys.insert(NAMED_KEY_HASH.to_string(), storage::new_uref(String::from("Empty")).into());
    named_keys
}