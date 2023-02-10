use crate::{constants, Error};
use alloc::string::{String, ToString};
use casper_contract::{
    contract_api::{
        runtime::{get_caller, self, call_contract},
        storage::{self, read},
    },
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{account::AccountHash, bytesrepr::{ToBytes, FromBytes}, CLTyped, URef, Key, U512, runtime_args, ContractHash, U256, ContractPackageHash, RuntimeArgs};
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
        },
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
    let caller = get_caller();
    let owner = get_named_key_by_name(constants::NAMED_KEY_OWNER);
    //let owner = get_key_val::<URef>(constants::NAMED_KEY_OWNER);
    let owner: AccountHash = storage::read(owner).unwrap_or_revert().unwrap_or_revert();
    if caller != owner {
        return false;
    }
    true
}
pub fn set_key<T>(key_name: &str, key_value: T)
where
    T: CLTyped + ToBytes,
{
    let key_uref = get_named_key_by_name(key_name);
    storage::write(key_uref, key_value);
}
pub fn get_key_val<T: FromBytes + CLTyped>(key: &str) -> T {
    let value: T = match runtime::get_key(key) {
        Some(Key::URef(uref)) => match storage::read(uref) {
            Ok(Some(value)) => value,
            Ok(None) => runtime::revert(Error::MissingValue),
            Err(error) => runtime::revert(Error::StorageError),
        },
        _ => runtime::revert(Error::MissingKey),
    };
    value
}

pub fn clear_all(){
    set_key(constants::NAMED_KEY_SECRET, "".to_string());
    set_key(constants::NAMED_KEY_HASH, "".to_string());
    set_key(constants::NAMED_KEY_START_TIME, 0);
    set_key(constants::NAMED_KEY_END_TIME, 0);
    set_key(constants::NAMED_KEY_OWNER, AccountHash::new([0u8; 32]));
    set_key(constants::NAMED_KEY_AMOUNT, U512::from(0));
    set_key(constants::NAMED_KEY_CONTRACT_HASH, "".to_string());
    set_key(constants::NAMED_KEY_HASH_TYPE, "".to_string());
    set_key(constants::NAMED_KEY_TYPE, "".to_string());
    set_key(constants::NAMED_KEY_RECIVER, AccountHash::new([0u8; 32]));
    // Note that the purse is not cleared, as it is owned by the contract and can be used for other times.
}

pub fn get_allowance() -> U256 {
    // contract hash of ERC20 token
    let contract_hash = get_key_val::<ContractHash>(constants::NAMED_KEY_CONTRACT_HASH);
    let owner = get_key_val::<AccountHash>(constants::NAMED_KEY_OWNER);
    let owner_key = Key::Account(owner);
    // get the contract-package-hash of our contract
    let contract_package_hash = get_key_val::<ContractPackageHash>(constants::NAMED_KEY_OWN_CONTRACT_PACKAGE_HASH);
    let mut runtimeargs = RuntimeArgs::new();
    runtimeargs.insert("owner", owner_key);
    runtimeargs.insert("spender", contract_package_hash);
    let allow = call_contract::<U256>(contract_hash, "allowance", runtimeargs);
    allow
}

pub fn get_balance() -> U256{
    let contract_hash = get_key_val::<ContractHash>(constants::NAMED_KEY_CONTRACT_HASH);
    let owner = get_key_val::<AccountHash>(constants::NAMED_KEY_OWNER);
    let owner_key = Key::Account(owner);
    let mut runtimeargs = RuntimeArgs::new();
    runtimeargs.insert("address", owner_key);
    let ballance = call_contract::<U256>(contract_hash, "balance_of", runtimeargs);
    ballance
}