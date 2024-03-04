// Edit metric metadata returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_metrics::MetricsAPI;
use datadog_api_client::datadogV1::model::*;
use std::ops::Add;
use std::time::{
    Duration,
    SystemTime,
    UNIX_EPOCH,
};

#[tokio::main]
async fn main() {
    let body =
        MetricMetadata::new().per_unit("second".to_string()).type_("count".to_string()).unit("byte".to_string());
    let configuration = Configuration::new();
    let api = MetricsAPI::with_config(configuration);
    let resp = api.update_metric_metadata(body).await;
    if let Ok(Some(value)) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
