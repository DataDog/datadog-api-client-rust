// Get hourly usage by product family returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_usage_metering::UsageMeteringAPI;
use datadog_api_client::datadogV2::model::*;
use std::ops::Add;
use std::time::{
    Duration,
    SystemTime,
    UNIX_EPOCH,
};

#[tokio::main]
async fn main() {
    let configuration = Configuration::new();
    let api = UsageMeteringAPI::with_config(configuration);
    let resp = api.get_hourly_usage().await;
    if let Ok(Some(value)) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
