// Get a span-based metric returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_spans_metrics::SpansMetricsAPI;
use datadog_api_client::datadogV2::model::*;
use std::ops::Add;
use std::time::{
    Duration,
    SystemTime,
    UNIX_EPOCH,
};

#[tokio::main]
async fn main() {
    // there is a valid "spans_metric" in the system
    let spans_metric_data_id = std::env::var("SPANS_METRIC_DATA_ID").unwrap();
    let configuration = Configuration::new();
    let api = SpansMetricsAPI::with_config(configuration);
    let resp = api.get_spans_metric().await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
