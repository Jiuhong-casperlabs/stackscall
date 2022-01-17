#![no_main]
#![no_std]

extern crate alloc;
use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec,
};
use casper_contract::contract_api::{runtime, storage};
use casper_types::{
    system::CallStackElement, CLType, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints,
    Key,
};

#[no_mangle]
fn test1() {
    let stacks = runtime::get_call_stack();
    for (i, stack) in stacks.into_iter().enumerate() {
        let key = String::from("stack") + &i.to_string();

        // runtime::put_key(key, stack);
        match stack {
            CallStackElement::Session { account_hash } => {
                runtime::put_key(key.as_str(), account_hash.into())
            }
            CallStackElement::StoredSession {
                account_hash,
                contract_package_hash,
                contract_hash,
            } => {
                let mut map2: BTreeMap<String, Key> = BTreeMap::new();
                let key1 = String::from("account_hash");
                let key2 = String::from("contract_package_hash");
                let key3 = String::from("contract_hash");

                //store purse into contract named_keys
                map2.insert(key1, account_hash.into());
                map2.insert(key2, contract_package_hash.into());
                map2.insert(key3, contract_hash.into());
                runtime::put_key(key.as_str(), storage::new_uref(map2).into());
            }

            CallStackElement::StoredContract {
                contract_package_hash,
                contract_hash,
            } => {
                let mut map3: BTreeMap<String, Key> = BTreeMap::new();
                let key1 = String::from("contract_package_hash");
                let key2 = String::from("contract_hash");

                //store purse into contract named_keys
                map3.insert(key1, contract_package_hash.into());
                map3.insert(key2, contract_hash.into());
                runtime::put_key(key.as_str(), storage::new_uref(map3).into());
            }
        };
    }
}

#[no_mangle]
fn call() {
    let mut entry_points = EntryPoints::new();

    let entrypoint1 = EntryPoint::new(
        "test1",
        vec![],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );

    entry_points.add_entry_point(entrypoint1);

    let (contracthash, _) = storage::new_contract(entry_points, None, None, None);

    runtime::put_key("targetcontract", contracthash.into());
}
