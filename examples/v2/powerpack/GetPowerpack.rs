// Get a Powerpack returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_powerpack::PowerpackAPI;
use datadog_api_client::datadogV2::model::*;
use std::ops::Add;
use std::time::{
    Duration,
    SystemTime,
    UNIX_EPOCH,
};

#[tokio::main]
async fn main() {
    // there is a valid "powerpack" in the system
    let powerpack_data_id = std::env::var("POWERPACK_DATA_ID").unwrap();
    let configuration = Configuration::new();
    let api = PowerpackAPI::with_config(configuration);
    let resp = api.get_powerpack().await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
