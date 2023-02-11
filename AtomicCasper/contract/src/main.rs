#![no_std]
#![no_main]
pub mod constants;
mod erc20util;
mod native_util;
mod nftutil;
mod transfers;
mod utils;
#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");
// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.
extern crate alloc;
use alloc::{
    collections::BTreeSet,
    string::{String, ToString},
    vec::Vec,
};
use casper_contract::{
    contract_api::{
        runtime::{self, get_caller},
        storage,
        system::{create_purse, get_purse_balance},
    },
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    runtime_args, AccessRights, ApiError, CLValue, ContractHash, ContractPackageHash, Key,
    RuntimeArgs, URef, U256, U512,
};
use constants::{get_entrypoints, get_named_keys};
use utils::{
    check_hash_type, check_ownership, generate_hash, get_current_time, get_key_val, is_timed_out,
    set_end_time, set_key, set_start_time, ToKey,
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
    ContractAlreadyInitialized = 24,
    NFTsNotOwnedByContract = 25,
    ReciverNotAnAccount = 26,
    OwnerNotSet = 27,
    OwnerReadError = 28,
    NativeTransferFailed = 29,
    RuntimeArgFailed = 30,
}
impl From<Error> for ApiError {
    fn from(error: Error) -> Self {
        ApiError::User(error as u16)
    }
}
pub type TokenId = U256;

#[no_mangle]
pub extern "C" fn get_hash() {
    let hash = get_key_val::<String>(constants::NAMED_KEY_HASH);
    runtime::ret(CLValue::from_t(hash).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn refund() {
    if !check_ownership() {
        runtime::revert(Error::AccessDenied);
    }
    if !is_timed_out() {
        runtime::revert(Error::EndTimeNotReached);
    }
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
    let hash_type = get_key_val::<String>(constants::NAMED_KEY_HASH_TYPE);
    let _secret_hash = generate_hash(hash_type.as_str(), secret.as_str());
    transfers::transfer_to(_secret_hash.as_str());
    utils::clear_all();
}

#[no_mangle]
pub extern "C" fn initiate() {
    // only the owner can initiate or reset the contract
    if !check_ownership() {
        runtime::revert(Error::AccessDenied);
    }
    // If the hash is already set, then the contract is already initialized, so we should revert
    if utils::already_initialized() {
        runtime::revert(Error::ContractAlreadyInitialized);
    }
    // set hash and hash_type :
    let hash = runtime::get_named_arg::<String>(constants::ARG_HASH);
    let hash_type = runtime::get_named_arg::<String>(constants::ARG_HASH_TYPE);
    if !check_hash_type(hash_type.as_str()) {
        runtime::revert(Error::HashTypeNotSupported);
    }
    set_key(constants::NAMED_KEY_HASH, hash);
    set_key(constants::NAMED_KEY_HASH_TYPE, hash_type);

    // Get the recipient
    let reciver = runtime::get_named_arg::<Key>(constants::ARG_RECEIVER);
    set_key(constants::NAMED_KEY_RECIVER, reciver);

    {
        // Get timeout and set start_time and end_time
        let timeout = runtime::get_named_arg::<u64>(constants::ARG_TIMEOUT);
        let current_time = get_current_time();
        set_start_time(current_time);
        set_end_time(current_time + timeout);
    }

    let type_ = get_key_val::<String>(constants::NAMED_KEY_TYPE);
    if type_ != "NFT" && type_ != "ERC-20" && type_ != "Direct" && type_ != "Custom" {
        runtime::revert(Error::TypeNotSupported);
    }
    // for NFT and ERC20 and Custom, we need to set the other contract hash
    if type_.as_str() != "Direct" {
        let contract_hash = runtime::get_named_arg::<String>(constants::ARG_CONTRACT_HASH);
        set_key(constants::NAMED_KEY_CONTRACT_HASH, contract_hash);
    }
    match type_.as_str() {
        "NFT" => {
            // ___________NFT__________________
            // 1. token_ids : Done
            // 2. hash : Done
            // 3. hash_type : Done
            // 4. type : Done
            // 5. owner : Done Before calling this function
            // 6. contract_hash : Done
            // 7. Recipient : Done
            // 8. timeout : Done
            // 9. amount : not needed
            // 10. purse : not needed
            // ___________NFT__________________
            let token_ids = runtime::get_named_arg::<Vec<TokenId>>(constants::ARG_TOKEN_IDS);
            set_key(constants::NAMED_KEY_TOKEN_IDS, token_ids.clone());
            // Check that the given token_ids are owned by our contract
            if !nftutil::check_nfts_ownership(token_ids) {
                runtime::revert(Error::NFTsNotOwnedByContract);
            }
        }
        "ERC-20" => {
            // Check if the contract has enough balance
            let contract_own_balance = erc20util::get_own_contract_balance();
            let amount = runtime::get_named_arg::<U256>(constants::ARG_AMOUNT);
            if contract_own_balance < amount.as_u64() {
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
        }
        "Custom" => {
            //TODO
        }
        _ => {
            runtime::revert(Error::TypeNotSupported);
        }
    }
}

#[no_mangle]
pub extern "C" fn get_deposit_purse() {
    if !check_ownership() {
        runtime::revert(Error::AccessDenied);
    }
    let deposit = utils::get_contracts_purse();
    // Next line is a bit sus!
    deposit.access_rights().set(AccessRights::READ_ADD, true);
    runtime::ret(CLValue::from_t(deposit).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn init() {
    // put the contract package hash into the named keys
    let contract_package_hash = runtime::get_named_arg::<ContractPackageHash>(
        constants::NAMED_KEY_OWN_CONTRACT_PACKAGE_HASH,
    );
    let contract_package_hash_uref = storage::new_uref(contract_package_hash);
    runtime::put_key(
        constants::NAMED_KEY_OWN_CONTRACT_PACKAGE_HASH,
        contract_package_hash_uref.into(),
    );

    let contract_hash =
        runtime::get_named_arg::<ContractHash>(constants::NAMED_KEY_OWN_CONTRACT_HASH);
    let contract_hash_uref = storage::new_uref(contract_hash);
    runtime::put_key(
        constants::NAMED_KEY_OWN_CONTRACT_HASH,
        contract_hash_uref.into(),
    );

    let type_ = runtime::get_named_arg::<String>(constants::ARG_TYPE);
    let type_uref = storage::new_uref(type_);
    runtime::put_key(constants::NAMED_KEY_TYPE, type_uref.into());

    let caller = get_caller().to_key();
    let owner = storage::new_uref(caller);
    runtime::put_key(constants::NAMED_KEY_OWNER, owner.into());
    let purse = create_purse();
    runtime::put_key(constants::NAMED_KEY_PURSE, purse.into());
}

fn install_contract() {
    let type_ = runtime::get_named_arg::<String>(constants::ARG_TYPE);
    let entry_points = get_entrypoints();
    let named_keys = get_named_keys();
    let (contract_hash, _contract_version) = storage::new_contract(
        entry_points,
        Some(named_keys),
        Some("atomic_swap_package_hash".to_string()),
        None,
    );
    let package_hash = ContractPackageHash::new(
        runtime::get_key("atomic_swap_package_hash")
            .unwrap_or_revert()
            .into_hash()
            .unwrap_or_revert(),
    );
    let constructor_access: URef =
        storage::create_contract_user_group(package_hash, "constructor", 1, Default::default())
            .unwrap_or_revert()
            .pop()
            .unwrap_or_revert();
    let _: () = runtime::call_contract(
        contract_hash,
        "init",
        runtime_args! {
            constants::NAMED_KEY_OWN_CONTRACT_PACKAGE_HASH => package_hash,
            constants::NAMED_KEY_OWN_CONTRACT_HASH => contract_hash,
            constants::ARG_TYPE => type_,
        },
    );
    let mut urefs = BTreeSet::new();
    urefs.insert(constructor_access);
    storage::remove_contract_user_group_urefs(package_hash, "constructor", urefs)
        .unwrap_or_revert();
    runtime::put_key("atomic_swap_contract", contract_hash.into());
}

#[no_mangle]
pub extern "C" fn call() {
    install_contract();
}

// **NOTE** : the key must go public after the transition of tokens was successfull!!

// **NOTE** : Important :: In NFT or ERC20 mode, the tokens (NFTs or ERC20 amount) must be "TRANSFERRED" to the contract, not "APPROVED" before calling "initiate" function
// Because approving will cause some security issues!!!!

/*
    let contracthash =  ContractHash::from_formatted_str("contract-300094544205F5F99Aa33CD87D8f0F0B391e0E6bc1cfB0ccFbF35067E6faB1F8")
        .unwrap();
    let contractpackagehash = ContractPackageHash::from_formatted_str("contract-package-wasm1ddc8ECf041E3A32C5E92155FE6a8437A55eA0716f2b9d9d2C4Da890a5d9621d")
    .unwrap();
*/
