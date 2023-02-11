use casper_contract::{contract_api::{system::transfer_from_purse_to_account, runtime}, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{Key, U512};
use crate::{constants, utils::{get_key_val, self}, Error};

pub fn transfer_native_tokens_to(reciver : Key){
    // transfer amount to reciver from contract's purse
    let amount = get_key_val::<U512>(constants::NAMED_KEY_AMOUNT);
    let reciver_account_hash = match reciver {
        Key::Account(account_hash) => account_hash,
        _ => runtime::revert(Error::ReciverNotAnAccount)
    };
    let contract_purse = utils::get_contracts_purse();
    transfer_from_purse_to_account(contract_purse, reciver_account_hash, amount, None).unwrap_or_revert_with(Error::NativeTransferFailed);
}

pub fn transfer_native_tokens_back(){
    let reciver = get_key_val::<Key>(constants::NAMED_KEY_OWNER);
    transfer_native_tokens_to(reciver);
}