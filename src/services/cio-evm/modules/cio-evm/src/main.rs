#![allow(non_snake_case)]
use marine_rs_sdk::marine;
use marine_rs_sdk::module_manifest;
// use cio_response_types::AMResponse;
module_manifest!();

mod eth;
mod utils;

pub fn main() {}

#[marine]
pub fn read_from_rpc(abi: &str, contract: String, method: String, args: Vec<String>, rpc_url: &String) -> String {
    
    eth::call(abi, contract, method, args, rpc_url)
}


// fn vault_path(filename: &str) -> String {
//     let cp = marine_rs_sdk::get_call_parameters();
//     format!("/tmp/vault/{}-{}/{}", cp.particle.id, cp.particle.token, filename)
// }