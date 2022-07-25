#![no_std]
use stellar_contract_sdk::{contractimpl, Env, Symbol};

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn put(e: Env, key: Symbol, val: Symbol) {
        e.put_contract_data(key, val)
    }

    pub fn del(e: Env, key: Symbol) {
        e.del_contract_data(key)
    }
}