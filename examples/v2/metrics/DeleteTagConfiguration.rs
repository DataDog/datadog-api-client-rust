// Delete a tag configuration returns "No Content" response
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
    let configuration = Configuration::new();
    let api = MetricsAPI::with_config(configuration);
    let resp = api.delete_tag_configuration().await;
    if let Ok(Some(value)) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
