use move_core_types::account_address::AccountAddressParseError;
use move_core_types::effects::Op;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use std::path::PathBuf;
use std::{
    collections::BTreeMap
};

use move_package::BuildConfig as MoveBuildConfig;
use move_package::Architecture;
use move_core_types::account_address::AccountAddress;

use sui_move::Command;
use sui_move::execute_move_command;

#[wasm_bindgen]
pub fn compile(
    path_str: String,
    // build_config: MoveBuildConfig,
    dev_mode: bool,
    test_mode: bool,
    generate_docs: bool,
    generate_abis: bool,
    install_dir: String,
    force_recompilation: bool,
    lock_file: String,
    additional_names: Vec<JsValue>,
    additional_addrs: Vec<JsValue>,
    fetch_deps_only: bool,
    skip_fetch_latest_git_deps: bool,
)  -> String {

    let path: Option<PathBuf> = Some(path_str.into());


    let typed_named_addr_map:BTreeMap<String, AccountAddress> = BTreeMap::new();

    for i in 0..additional_names.len() {
        let address: Result<AccountAddress, AccountAddressParseError>
         = AccountAddress::from_hex_literal(&additional_addrs[i].as_string().unwrap());

        typed_named_addr_map.insert(additional_names[i].as_string().unwrap(), address.unwrap());
    }

    let optioned_install_dir: Option<PathBuf>;
    if install_dir.len() > 0 {
        optioned_install_dir = None;
    }
    else {
        optioned_install_dir = Some(install_dir.into());
    }

    let optioned_lock_file: Option<PathBuf>;
    if lock_file.len() > 0 {
        optioned_lock_file = None;
    }
    else {
        optioned_lock_file = Some(lock_file.into());
    }

    let build_config: MoveBuildConfig = MoveBuildConfig {
        dev_mode: dev_mode,
        test_mode: test_mode,
        generate_docs: generate_docs,
        generate_abis: generate_abis,
        install_dir: optioned_install_dir,
        force_recompilation: force_recompilation,
        lock_file: optioned_lock_file,
        additional_named_addresses: typed_named_addr_map,
        architecture: Some(Architecture::Move),
        fetch_deps_only: fetch_deps_only,
        skip_fetch_latest_git_deps: skip_fetch_latest_git_deps
    };

    let res_string = String::new();
    match execute_move_command(path, build_config, Command::Build){
        Ok(result) => {
            res_string.push_str(result);
        }

        Err(error) => {
            res_string.push_str(error);
        }
    }
    return res_string
}