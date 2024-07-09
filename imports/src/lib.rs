use marine_rs_sdk::marine;

#[marine]
#[module_import("cio_evm")]
extern "C" {

    pub fn prepare_eth_call(
        contract_address: String,
        function_name: String, 
        args: Vec<String>
    ) -> String;

    pub fn parse_result(
        raw_result: String
    ) -> String;

    pub fn decode_address(
        topic: String
    ) -> String;

    pub fn decode_string(
        topic: String
    ) -> String;

    pub fn prepare_create_filter(
        contract_address: String, 
        topic: String, 
        latest_block: String, 
    ) -> String;

    pub fn prepare_poll_filter(
        filter: String, 
    ) -> String;

}