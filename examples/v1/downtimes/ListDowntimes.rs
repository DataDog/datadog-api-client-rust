// Get all downtimes returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_downtimes::DowntimesAPI;
use datadog_api_client::datadogV1::model::*;
use std::ops::Add;
use std::time::{
    Duration,
    SystemTime,
    UNIX_EPOCH,
};

#[tokio::main]
async fn main() {
    let configuration = Configuration::new();
    let api = DowntimesAPI::with_config(configuration);
    let resp = api.list_downtimes().await;
    if let Ok(Some(value)) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
