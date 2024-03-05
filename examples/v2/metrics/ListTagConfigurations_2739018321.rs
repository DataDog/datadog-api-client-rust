// Get a list of metrics with configured filter returns "Success" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_metrics::MetricsAPI;
use datadog_api_client::datadogV2::model::*;

#[tokio::main]
async fn main() {
    let configuration = Configuration::new();
    let api = MetricsAPI::with_config(configuration);
    let resp = api.list_tag_configurations().await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
