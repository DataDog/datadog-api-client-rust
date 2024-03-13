// Get an AuthN Mapping by UUID returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_authn_mappings::*;

#[tokio::main]
async fn main() {
    // there is a valid "authn_mapping" in the system
    let authn_mapping_data_id = std::env::var("AUTHN_MAPPING_DATA_ID").unwrap();
    let configuration = Configuration::new();
    let api = AuthNMappingsAPI::with_config(configuration);
    let resp = api.get_authn_mapping(authn_mapping_data_id.clone()).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}