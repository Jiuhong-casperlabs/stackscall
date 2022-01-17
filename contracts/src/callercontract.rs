#![no_main]
#![no_std]

extern crate alloc;
use alloc::{vec, string::String};
use casper_contract::contract_api::{runtime, storage};
use casper_types::{
    self, CLType, ContractHash, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints,
    Parameter, RuntimeArgs,
};

const ARG_TARGET_HASH: &str = "targethash";

#[no_mangle]
fn callcontract() {
    let target_hash_str: String = runtime::get_named_arg(ARG_TARGET_HASH);
    
    let targethash = ContractHash::from_formatted_str(&target_hash_str).unwrap();

    runtime::call_contract(targethash, "test1", RuntimeArgs::default())
}

#[no_mangle]
fn call() {
    let mut entry_points = EntryPoints::new();

    let entry_point = EntryPoint::new(
        "callcontract",
        vec![Parameter::new(ARG_TARGET_HASH, CLType::String)],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );

    entry_points.add_entry_point(entry_point);

    let (contracthash, _) = storage::new_contract(entry_points, None, None, None);

    runtime::put_key("callstackcontract", contracthash.into());
}
