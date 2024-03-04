// List tag configuration by name returns "Success" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_metrics::MetricsAPI;
use datadog_api_client::datadogV2::model::*;
use std::ops::Add;
use std::time::{
    Duration,
    SystemTime,
    UNIX_EPOCH,
};

#[tokio::main]
async fn main() {
    // there is a valid "metric_tag_configuration" in the system
    let metric_tag_configuration_data_id = std::env::var("METRIC_TAG_CONFIGURATION_DATA_ID").unwrap();
    let configuration = Configuration::new();
    let api = MetricsAPI::with_config(configuration);
    let resp = api.list_tag_configuration_by_name().await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
