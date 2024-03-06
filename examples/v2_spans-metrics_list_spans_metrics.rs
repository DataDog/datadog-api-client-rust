// Get all span-based metrics returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_spans_metrics::*;

#[tokio::main]
async fn main() {
    let configuration = Configuration::new();
    let api = SpansMetricsAPI::with_config(configuration);
    let resp = api.list_spans_metrics().await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}