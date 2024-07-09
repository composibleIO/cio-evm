// use marine_rs_sdk::get_call_parameters;
// use chrono::{Utc};
// use std::fs;
// use serde_json::Value;
// use std::path::{PathBuf};
// use ethers_contract_derive::{abigen};
use hex::{encode as hex_encode};
// use cio_curl_effector_imports::HttpHeader;


// use ethabi::{Token, encode, ethereum_types::H256};
use ethabi_contract::use_contract;
// use ethabi_derive;


pub fn call(_abi: &str, contract: String, _method: String, _args: Vec<String>, rpc_url: &String) -> String {

    use_contract!(MyContract, "../../contract.json");

    // println!("{:?}", MyContract);

    let nonce = crate::utils::get_nonce();

    let data: String = crate::utils::EthRequest::new(
        
        rpc_url.clone(), 
        String::from("eth_call"), 
        nonce
    
    ).format_call(

        &contract, 
        &format!(
            "0x{}", 
            hex_encode(
                "kio"
              
                // MyContract::functions::config::encode_input(
                //     args
                // )
            )
        )
    );


    data

   
}