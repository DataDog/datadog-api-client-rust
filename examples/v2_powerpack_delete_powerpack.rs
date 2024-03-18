// Delete a powerpack returns "OK" response
use chrono::prelude::{DateTime, Utc};
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_powerpack::*;
use datadog_api_client::datadogV2::model::*;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    // there is a valid "powerpack" in the system
    let powerpack_data_id = std::env::var("POWERPACK_DATA_ID").unwrap();
    let configuration = Configuration::new();
    let api = PowerpackAPI::with_config(configuration);
    let resp = api.delete_powerpack(powerpack_data_id.clone()).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
