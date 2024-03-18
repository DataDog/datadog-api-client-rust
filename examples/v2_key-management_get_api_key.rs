// Get API key returns "OK" response
use chrono::prelude::{DateTime, Utc};
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_key_management::*;
use datadog_api_client::datadogV2::model::*;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    // there is a valid "api_key" in the system
    let api_key_data_id = std::env::var("API_KEY_DATA_ID").unwrap();
    let configuration = Configuration::new();
    let api = KeyManagementAPI::with_config(configuration);
    let resp = api
        .get_api_key(api_key_data_id.clone(), GetAPIKeyOptionalParams::default())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
