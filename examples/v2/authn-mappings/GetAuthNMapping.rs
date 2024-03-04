// Get an AuthN Mapping by UUID returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_authn_mappings::AuthNMappingsAPI;
use datadog_api_client::datadogV2::model::*;
use std::ops::Add;
use std::time::{
    Duration,
    SystemTime,
    UNIX_EPOCH,
};

#[tokio::main]
async fn main() {
    // there is a valid "authn_mapping" in the system
    let authn_mapping_data_id = std::env::var("AUTHN_MAPPING_DATA_ID").unwrap();
    let configuration = Configuration::new();
    let api = AuthNMappingsAPI::with_config(configuration);
    let resp = api.get_authn_mapping().await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
