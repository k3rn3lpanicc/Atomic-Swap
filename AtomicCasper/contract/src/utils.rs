use crate::{constants, Error, TokenId};
use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use casper_contract::{
    contract_api::{
        runtime::{self, get_call_stack},
        storage::{self, read},
    },
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    account::AccountHash,
    bytesrepr::{FromBytes, ToBytes},
    system::CallStackElement,
    CLTyped, ContractHash, ContractPackageHash, Key, URef, U512,
};
pub fn generate_hash(hash_type: &str, secret: &str) -> String {
    use sha3::Digest;
    match hash_type {
        "sha3-256" => {
            let mut hasher = sha3::Sha3_256::new();
            hasher.update(secret);
            hex::encode(hasher.finalize())
        }
        "sha3-512" => {
            let mut hasher = sha3::Sha3_512::new();
            hasher.update(secret);
            hex::encode(hasher.finalize())
        }
        "Keccak256" => {
            let mut hasher = sha3::Keccak256::new();
            hasher.update(secret);
            hex::encode(hasher.finalize())
        }
        "Keccak512" => {
            let mut hasher = sha3::Keccak512::new();
            hasher.update(secret);
            hex::encode(hasher.finalize())
        }
        "blake2b" => {
            let bytes = runtime::blake2b(secret.as_bytes());
            hex::encode(bytes)
        }
        _ => "".to_string(),
    }
}

pub fn check_hash_type(hash_type: &str) -> bool {
    if hash_type == "sha3-256"
        || hash_type == "sha3-512"
        || hash_type == "Keccak256"
        || hash_type == "Keccak512"
        || hash_type == "blake2b"
    {
        return true;
    }
    false
}

pub fn get_named_key_by_name(dict_name: &str) -> casper_types::URef {
    casper_contract::contract_api::runtime::get_key(dict_name)
        .unwrap_or_revert_with(Error::KeyNotFound)
        .into_uref()
        .unwrap_or_revert_with(Error::KeyNotFound)
}
pub fn get_current_time() -> u64 {
    casper_contract::contract_api::runtime::get_blocktime().into()
}
pub fn _get_start_time() -> u64 {
    let start_time = get_named_key_by_name(constants::NAMED_KEY_START_TIME);
    read(start_time)
        .unwrap_or_revert_with(Error::StartTimeNotSet)
        .unwrap_or_revert_with(Error::StartTimeReadError)
}
pub fn set_start_time(start_time: u64) {
    let start_time_uref = get_named_key_by_name(constants::NAMED_KEY_START_TIME);
    storage::write(start_time_uref, start_time);
}
pub fn get_end_time() -> u64 {
    let end_time = get_named_key_by_name(constants::NAMED_KEY_END_TIME);
    read(end_time)
        .unwrap_or_revert_with(Error::EndTimeNotSet)
        .unwrap_or_revert_with(Error::EndTimeReadError)
}
pub fn set_end_time(end_time: u64) {
    let end_time_uref = get_named_key_by_name(constants::NAMED_KEY_END_TIME);
    storage::write(end_time_uref, end_time);
}
pub fn is_timed_out() -> bool {
    get_current_time() > get_end_time()
}
pub fn _is_started() -> bool {
    get_current_time() > _get_start_time()
}

pub fn check_ownership() -> bool {
    let caller = get_caller_key();
    let owner = get_owner();
    if caller != owner {
        return false;
    }
    true
}
pub fn get_owner() -> Key {
    let owner = get_named_key_by_name(constants::NAMED_KEY_OWNER);
    read(owner)
        .unwrap_or_revert_with(Error::OwnerNotSet)
        .unwrap_or_revert_with(Error::OwnerReadError)
}

pub fn set_key<T>(key_name: &str, key_value: T)
where
    T: CLTyped + ToBytes,
{
    if runtime::get_key(key_name).is_none() {
        let key_uref = storage::new_uref(key_value).into();
        runtime::put_key(key_name, key_uref);
    } else {
        let key_uref = get_named_key_by_name(key_name);
        storage::write(key_uref, key_value);
    }
}
pub fn get_key_val<T: FromBytes + CLTyped>(key: &str) -> T {
    let value: T = match runtime::get_key(key) {
        Some(Key::URef(uref)) => match storage::read(uref) {
            Ok(Some(value)) => value,
            Ok(None) => runtime::revert(Error::MissingValue),
            Err(_error) => runtime::revert(Error::StorageError),
        },
        _ => runtime::revert(Error::MissingKey),
    };
    value
}

pub fn clear_all() {
    set_key(constants::NAMED_KEY_SECRET, "".to_string());
    set_key(constants::NAMED_KEY_HASH, "".to_string());
    set_key::<u64>(constants::NAMED_KEY_START_TIME, 0);
    set_key::<u64>(constants::NAMED_KEY_END_TIME, 0);
    set_key(
        constants::NAMED_KEY_OWNER,
        AccountHash::new([0u8; 32]).to_key(),
    );
    set_key(constants::NAMED_KEY_AMOUNT, U512::from(0u64));
    set_key(constants::NAMED_KEY_CONTRACT_HASH, "".to_string());
    set_key(constants::NAMED_KEY_HASH_TYPE, "".to_string());
    set_key(constants::NAMED_KEY_TYPE, "".to_string());
    set_key(
        constants::NAMED_KEY_RECIVER,
        Key::Account(AccountHash::new([0u8; 32])),
    );
    let empty_vec: Vec<TokenId> = Vec::new();
    set_key(constants::NAMED_KEY_TOKEN_IDS, empty_vec);
    // Note that the purse is not cleared, as it is owned by the contract and can be used for other times.
}

pub fn _get_contract_hash() -> ContractHash {
    get_key_val::<ContractHash>(constants::NAMED_KEY_OWN_CONTRACT_HASH)
}

pub fn get_contract_package_hash() -> ContractPackageHash {
    get_key_val::<ContractPackageHash>(constants::NAMED_KEY_OWN_CONTRACT_PACKAGE_HASH)
}

pub trait ToKey {
    fn to_key(&self) -> Key;
}
impl ToKey for AccountHash {
    fn to_key(&self) -> Key {
        Key::Account(*self)
    }
}
impl ToKey for ContractHash {
    fn to_key(&self) -> Key {
        Key::Hash(self.value())
    }
}
impl ToKey for ContractPackageHash {
    fn to_key(&self) -> Key {
        Key::Hash(self.value())
    }
}

pub fn already_initialized() -> bool {
    let hash_uref = runtime::get_key(constants::NAMED_KEY_HASH);
    if hash_uref.is_none() {
        return false;
    }
    let hash_uref = hash_uref.unwrap_or_revert().into_uref().unwrap_or_revert();
    let hash = storage::read::<String>(hash_uref)
        .unwrap_or_revert()
        .unwrap_or_revert();
    if hash.as_str() == "" {
        return false;
    }
    true
}

pub fn get_contracts_purse() -> URef {
    let deposit = get_named_key_by_name(constants::NAMED_KEY_PURSE);
    let deposit: URef = storage::read(deposit)
        .unwrap_or_revert_with(Error::PurseNotSet)
        .unwrap_or_revert_with(Error::PurseReadError);
    deposit
}

pub fn get_caller_key() -> Key {
    let call_stack = get_call_stack();
    let caller = call_stack.get(call_stack.len() - 2);
    element_to_key(caller.unwrap_or_revert())
}
fn element_to_key(element: &CallStackElement) -> Key {
    match element {
        CallStackElement::Session { account_hash } => (*account_hash).into(),
        CallStackElement::StoredSession {
            account_hash,
            contract_package_hash: _,
            contract_hash: _,
        } => (*account_hash).into(),
        CallStackElement::StoredContract {
            contract_package_hash,
            contract_hash: _,
        } => (*contract_package_hash).into(),
    }
}
