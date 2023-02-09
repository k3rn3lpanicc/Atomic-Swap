use casper_contract::{unwrap_or_revert::UnwrapOrRevert, contract_api::{storage::{read, self}, runtime::get_caller}};
use casper_types::{bytesrepr::{ToBytes, FromBytes}, CLTyped, account::AccountHash};
use crate::{constants, Error};
pub fn get_named_key_by_name(dict_name : &str) -> casper_types::URef {
    casper_contract::contract_api::runtime::get_key(dict_name).unwrap_or_revert_with(Error::KeyNotFound).into_uref().unwrap_or_revert_with(Error::KeyNotFound)
}
pub fn get_current_time() -> u64 {
    casper_contract::contract_api::runtime::get_blocktime().into()
}
pub fn get_start_time() -> u64 {
    let start_time = get_named_key_by_name(constants::NAMED_KEY_START_TIME);
    read(start_time).unwrap_or_revert_with(Error::StartTimeNotSet).unwrap_or_revert_with(Error::StartTimeReadError)
}
pub fn set_start_time( start_time : u64) {
    let start_time_uref = get_named_key_by_name(constants::NAMED_KEY_START_TIME);
    storage::write(start_time_uref, start_time);
}
pub fn get_end_time() -> u64 {
    let end_time = get_named_key_by_name(constants::NAMED_KEY_END_TIME);
    read(end_time).unwrap_or_revert_with(Error::EndTimeNotSet).unwrap_or_revert_with(Error::EndTimeReadError)
}
pub fn set_end_time( end_time : u64) {
    let end_time_uref = get_named_key_by_name(constants::NAMED_KEY_END_TIME);
    storage::write(end_time_uref, end_time);
}
pub fn is_timed_out() -> bool {
    get_current_time() > get_end_time()
}
pub fn is_started() -> bool {
    get_current_time() > get_start_time()
}
pub fn check_ownership() -> bool {
    let caller = get_caller();
    let owner = get_named_key_by_name(constants::NAMED_KEY_OWNER);
    let owner : AccountHash = storage::read(owner).unwrap_or_revert().unwrap_or_revert();
    if caller != owner {
        return false;
    }
    true
}
pub fn set_key<T>(key_name : &str, key_value : T) 
    where 
        T: CLTyped + ToBytes,
{    
    let key_uref = get_named_key_by_name(key_name);
    storage::write(key_uref, key_value);
}

// pub fn has_key<T>(key_name : &str) -> bool
//     where 
//         T: CLTyped + ToBytes + FromBytes{
//     let key_uref = get_named_key_by_name(key_name);
//     storage::read::<T>(key_uref).unwrap_or_revert().is_some()
// }