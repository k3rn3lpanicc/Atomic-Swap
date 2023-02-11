use casper_contract::contract_api::runtime::call_contract;
use casper_types::{ContractHash, Key, RuntimeArgs, U256, U512, account::AccountHash};
use crate::{utils::{get_key_val, get_contract_hash, toKey, get_contract_package_hash}, constants, transfers::U512ToU256};
pub fn get_own_contract_balance() -> u64{
    let erc20_contract_hash = get_key_val::<ContractHash>(constants::NAMED_KEY_CONTRACT_HASH);
    let own_contract_hash = get_contract_package_hash();
    let own_contract_hash_key = own_contract_hash.to_key();
    let mut runtimeargs = RuntimeArgs::new();
    runtimeargs.insert("address", own_contract_hash_key);
    let ballance = call_contract::<U256>(erc20_contract_hash, "balance_of", runtimeargs);
    ballance.as_u64()
}
pub fn transfer_erc20_tokens_to(reciver : Key){
    let amount = get_key_val::<U512>(constants::NAMED_KEY_AMOUNT).to_u256();
    let erc20_contract_hash = get_key_val::<ContractHash>(constants::NAMED_KEY_CONTRACT_HASH);
    let mut runtimeargs = RuntimeArgs::new();
    runtimeargs.insert("amount", amount);
    runtimeargs.insert("recipient", reciver);
    let _ = call_contract::<()>(
        erc20_contract_hash,
        "transfer",
        runtimeargs
    );
}

pub fn transfer_back() {
    let reciver = get_key_val::<Key>(constants::NAMED_KEY_OWNER);
    transfer_erc20_tokens_to(reciver);
}