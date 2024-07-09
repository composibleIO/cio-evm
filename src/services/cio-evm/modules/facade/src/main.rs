#![allow(non_snake_case)]
use marine_rs_sdk::marine;
use marine_rs_sdk::module_manifest;
use cio_response_types::AMResponse;
use curl_effector_imports::{ CurlRequest, HttpHeader, curl_post};
use std::fs;
use serde_json::Value;
use marine_rs_sdk::get_call_parameters;
use chrono::Utc;
use std::path::PathBuf;




module_manifest!();


mod abi;

pub fn main() {}

// can this be generic or services per contract 
// include abi 

#[marine]
pub fn read_from_rpc(contract: String, method: String, args: Vec<String>, rpc_url: String) -> String { // AMResponse {
    
    let data = cio_evm_imports::read_from_rpc(abi::ABI, contract, method, args, &rpc_url);

    println!(data);


    // let h1 = HttpHeader {
    //     name: "Content-Type".to_string(),
    //     value: "application/json".to_string()
    // };
 
    // let request = CurlRequest {
    //     url: rpc_url.clone(),    
    //     headers: vec![h1]
    // };

    // let source_path = crate::vault_path("evm_body");
    // let target_path = crate::vault_path("evm_response");
    
    // let _ = fs::write(PathBuf::from(source_path.clone()), data);

    // let response = curl_post(request, source_path, target_path.clone());
    // let timestamp = Utc::now().timestamp_millis();
    // let cp = get_call_parameters();

    // if response.success {
    //     let result = fs::read_to_string(target_path.clone()).unwrap();
    //     let v: Value = serde_json::from_str(&result).unwrap();

    //     return AMResponse {
    //         success: true,
    //         result: v["IpfsHash"].to_string().replace("\"",""),
    //         result_raw: result,
    //         timestamp,
    //         host_id: cp.host_id
    //     }  

    // } else {

    //     return AMResponse {
    //         success: false,
    //         result_raw: String::from(""),
    //         result: response.error,
    //         timestamp,
    //         host_id: cp.host_id
    //     }  
    // }

    data
}

fn vault_path(filename: &str) -> String {
        let cp = marine_rs_sdk::get_call_parameters();
        format!("/tmp/vault/{}-{}/{}", cp.particle.id, cp.particle.token, filename)
    }


