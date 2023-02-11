use alloc::string::String;
use casper_contract::contract_api::runtime;
use casper_types::{Key, U256, U512};

use crate::{constants, erc20util, native_util, nftutil, utils::get_key_val, Error};

pub fn transfer_to(secret_hash: &str) {
    let saved_hash = get_key_val::<String>(constants::NAMED_KEY_HASH);
    if secret_hash != saved_hash.as_str() {
        runtime::revert(Error::HashMismatch);
    }
    let _type = get_key_val::<String>(constants::NAMED_KEY_TYPE);
    let _type = _type.as_str();
    let reciver = get_key_val::<Key>(constants::NAMED_KEY_RECIVER);
    match _type {
        "NFT" => {
            nftutil::transfer_to(reciver);
        }
        "ERC20" => {
            erc20util::transfer_erc20_tokens_to(reciver);
        }
        "Direct" => {
            native_util::transfer_native_tokens_to(reciver);
        }
        "Custom" => {}
        _ => {
            runtime::revert(Error::TypeNotFound);
        }
    }
}
pub fn transfer_back() {
    let _type = get_key_val::<String>(constants::NAMED_KEY_TYPE);
    let _type = _type.as_str();
    match _type {
        "NFT" => {
            nftutil::transfer_back();
        }
        "ERC20" => {
            erc20util::transfer_back();
        }
        "Direct" => {
            native_util::transfer_native_tokens_back();
        }
        "Custom" => {}
        _ => {
            runtime::revert(Error::TypeNotFound);
        }
    }
}
pub trait U512ToU256 {
    fn to_u256(self) -> U256;
}
impl U512ToU256 for U512 {
    fn to_u256(self) -> U256 {
        let mut result = U256::zero();
        result.0[..4].clone_from_slice(&self.0[..4]);
        result
    }
}
