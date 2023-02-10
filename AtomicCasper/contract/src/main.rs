/*
TODO: Change NDPC contract and add this : whenever a nft is transferred,
the contract should check if the token_id that is transferring to dst account
is already in another holder_id or not, if it wasn't in any of the dst account's holders,
then it should create a new holder_id for it, if it was in another holder_id, then it
should just add the amount to the existing holder_id.
*/
#![no_std]
#![no_main]
//mod ndpc_types;
//mod ndpc_utils;
pub mod constants;
mod utils;
mod transfers;
#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.
extern crate alloc;
use core::borrow::Borrow;
use alloc::{
    collections::BTreeSet,
    string::{String, ToString},
};
use casper_contract::{
    contract_api::{
        runtime::{self, get_caller},
        storage,
        system::{create_purse, get_purse_balance},
    },
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{AccessRights, ApiError, CLValue, ContractPackageHash, RuntimeArgs, URef, U512, Key, account::AccountHash, ContractHash, system::CallStackElement, runtime_args, U256};
use constants::{get_entrypoints, get_named_keys};
use utils::{
    check_hash_type, check_ownership, generate_hash, get_current_time, is_timed_out, set_end_time,
    set_key, set_start_time, get_key_val, get_allowance,
};
/// An error enum which can be converted to a `u16` so it can be returned as an `ApiError::User`.
#[repr(u16)]
pub enum Error {
    StartTimeNotSet = 0,
    StartTimeReadError = 1,
    EndTimeNotSet = 2,
    EndTimeReadError = 3,
    HashNotSet = 4,
    HashReadError = 5,
    AccessDenied = 6,
    TypeNotSupported = 7,
    ContractHashNotSet = 8,
    ContractHashReadError = 9,
    NotEnoughBalance = 10,
    PurseNotSet = 11,
    PurseReadError = 12,
    KeyNotFound = 13,
    EndTimeNotReached = 14,
    EndTimePassed = 15,
    HashTypeNotSupported = 16,
    MissingValue = 17,
    MissingKey = 18,
    HashMismatch = 19,
    TypeNotFound = 20,
    UnexpectedKeyVariant = 21,
    StorageError = 22,
    InvalidContext = 23,
}
impl From<Error> for ApiError {
    fn from(error: Error) -> Self {
        ApiError::User(error as u16)
    }
}

#[no_mangle]
pub extern "C" fn get_hash() {
    let hash = utils::get_named_key_by_name(constants::NAMED_KEY_HASH);
    let hash = get_key_val::<String>(constants::NAMED_KEY_HASH);
    runtime::ret(CLValue::from_t(hash).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn refund(){
    if !check_ownership() {
        runtime::revert(Error::AccessDenied);
    }
    if !is_timed_out() {
        runtime::revert(Error::EndTimeNotReached);
    }
    // 1. transfer back the tokens or nfts based on the type of the contract (NFT, ERC-20, Direct, Custom) in a different function
    // 2. Reset Everything (hash, hash_type, type, contract_hash, start_time, end_time, amount, purse) to default values
    transfers::transfer_back();
    utils::clear_all();    
}

#[no_mangle]
pub extern "C" fn unlock() {
    let secret = runtime::get_named_arg::<String>(constants::ARG_SECRET);
    if check_ownership() {
        runtime::revert(Error::AccessDenied);
    } 
    if is_timed_out() {
        runtime::revert(Error::EndTimePassed);
    }
    // transfer tokens or money
    let hash_type = get_key_val::<String>(constants::NAMED_KEY_HASH_TYPE);
    let _secret_hash = generate_hash(hash_type.as_str(), secret.as_str());
    transfers::transfer_to(_secret_hash.as_str());
    utils::clear_all();
}

#[no_mangle]
pub extern "C" fn initiate() {
    if !check_ownership() {
        runtime::revert(Error::AccessDenied);
    }
    let hash = runtime::get_named_arg::<String>(constants::ARG_HASH);
    let hash_type = runtime::get_named_arg::<String>(constants::ARG_HASH_TYPE);
    if !check_hash_type(hash_type.as_str()) {
        runtime::revert(Error::HashTypeNotSupported);
    }
    set_key(constants::NAMED_KEY_HASH, hash);
    set_key(constants::NAMED_KEY_HASH_TYPE, hash_type);
    let type_ = runtime::get_named_arg::<String>(constants::ARG_TYPE);
    if type_ != "NFT" && type_ != "ERC-20" && type_ != "Direct" && type_ != "Custom" {
        // NFT : NFT mode neeeds to set the contract_hash field
        // ERC-20 : ERC-20 mode needs to set the contract_hash field
        // Direct : Direct mode needs to send the amount to the contract (by a purse)
        runtime::revert(Error::TypeNotSupported);
    }
    // based on the type, go on, do not use if else, use match
    match type_.as_str() {
        "NFT" => {
            let contract_hash = runtime::get_named_arg::<ContractHash>(constants::ARG_CONTRACT_HASH);
            set_key(constants::NAMED_KEY_CONTRACT_HASH, contract_hash);
        }
        "ERC-20" => {
            // If contract is not able to spend the amount, or if the amount is more than user's balance, revert it!
            let balance = utils::get_balance().as_u64();
            let contract_hash = runtime::get_named_arg::<ContractHash>(constants::ARG_CONTRACT_HASH);
            set_key(constants::NAMED_KEY_CONTRACT_HASH, contract_hash);
            let amount = runtime::get_named_arg::<U256>(constants::ARG_AMOUNT);
            let allowed_amount = get_allowance();
            if allowed_amount.as_u64() < amount.as_u64() || balance < amount.as_u64(){
                runtime::revert(Error::NotEnoughBalance);
            }
        }
        "Direct" => {
            let amount = runtime::get_named_arg::<U512>(constants::ARG_AMOUNT);
            // Get the deposit purse from contract
            let deposit_uref = utils::get_named_key_by_name(constants::NAMED_KEY_PURSE);
            let deposit: URef = storage::read(deposit_uref)
                .unwrap_or_revert_with(Error::PurseNotSet)
                .unwrap_or_revert_with(Error::PurseReadError);
            // Read the balance of the deposit purse
            let balance = get_purse_balance(deposit);
            // Check if the balance exists
            if balance.is_none() {
                runtime::revert(Error::PurseNotSet);
            }
            // Check if the balance is enough
            let balance = balance.unwrap();
            let dif = balance.checked_sub(amount);
            if dif.is_none() {
                runtime::revert(Error::NotEnoughBalance);
            }
            let dif = dif.unwrap();
            if dif.as_u64() < 2500000000 {
                // revert if the balance is less than 2.5 CSPR
                runtime::revert(Error::NotEnoughBalance);
            }
            set_key(constants::NAMED_KEY_AMOUNT, amount);
        },
        "Custom" => {
            //TODO:
        }
        _ => {
            runtime::revert(Error::TypeNotSupported);
        }
    }
    set_key(constants::NAMED_KEY_TYPE, type_);
    let reciver = runtime::get_named_arg::<AccountHash>(constants::ARG_RECEIVER);
    set_key(constants::NAMED_KEY_RECIVER, reciver);
    let timeout = runtime::get_named_arg::<u64>(constants::ARG_TIMEOUT);
    set_start_time(get_current_time());
    set_end_time(get_current_time() + timeout);

}

#[no_mangle]
pub extern "C" fn get_deposit_purse() {
    if !check_ownership() {
        runtime::revert(Error::AccessDenied);
    }
    let deposit = utils::get_named_key_by_name(constants::NAMED_KEY_PURSE);
    let deposit: URef = storage::read(deposit)
        .unwrap_or_revert_with(Error::PurseNotSet)
        .unwrap_or_revert_with(Error::PurseReadError);
    deposit.access_rights().set(AccessRights::READ_ADD, true);
    runtime::ret(CLValue::from_t(deposit.into_add()).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn init() {
    // put the contract package hash into the named keys
    let contract_package_hash = runtime::get_named_arg::<ContractPackageHash>(constants::NAMED_KEY_OWN_CONTRACT_PACKAGE_HASH);
    let contract_package_hash_uref = storage::new_uref(contract_package_hash);
    runtime::put_key(constants::NAMED_KEY_OWN_CONTRACT_PACKAGE_HASH, contract_package_hash_uref.into());
    let caller = get_caller();
    let owner = storage::new_uref(caller);
    runtime::put_key(constants::NAMED_KEY_OWNER, owner.into());
    let purse = create_purse();
    runtime::put_key(constants::NAMED_KEY_PURSE, purse.into());

}

fn install_contract() {
    let entry_points = get_entrypoints();
    let named_keys = get_named_keys();
    let (contract_hash, _contract_version) = storage::new_contract(
        entry_points,
        Some(named_keys),
        Some("droplinked_atomic_swap_package_hash".to_string()),
        None,
    );
    let package_hash = ContractPackageHash::new(
        runtime::get_key("droplinked_atomic_swap_package_hash")
            .unwrap_or_revert()
            .into_hash()
            .unwrap_or_revert(),
    );
    let constructor_access: URef =
        storage::create_contract_user_group(package_hash, "constructor", 1, Default::default())
            .unwrap_or_revert()
            .pop()
            .unwrap_or_revert();
    let _: () = runtime::call_contract(contract_hash, "init", runtime_args! {
        constants::NAMED_KEY_OWN_CONTRACT_PACKAGE_HASH => package_hash,
    });
    let mut urefs = BTreeSet::new();
    urefs.insert(constructor_access);
    storage::remove_contract_user_group_urefs(package_hash, "constructor", urefs)
        .unwrap_or_revert();
    runtime::put_key("droplink_atomic_swap_contract", contract_hash.into());
}

#[no_mangle]
pub extern "C" fn call() {
    install_contract();
}

// **NOTE** : the key must go public after the transition of tokens was successfull!!

/*
    let contracthash =  ContractHash::from_formatted_str("contract-300094544205F5F99Aa33CD87D8f0F0B391e0E6bc1cfB0ccFbF35067E6faB1F8")
        .unwrap();
    let contractpackagehash = ContractPackageHash::from_formatted_str("contract-package-wasm1ddc8ECf041E3A32C5E92155FE6a8437A55eA0716f2b9d9d2C4Da890a5d9621d")
    .unwrap();
    */