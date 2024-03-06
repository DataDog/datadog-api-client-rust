// List distinct metric volumes by metric name returns "Success" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_metrics::*;

#[tokio::main]
async fn main() {
    let configuration = Configuration::new();
    let api = MetricsAPI::with_config(configuration);
    let resp = api
        .list_volumes_by_metric_name("static_test_metric_donotdelete".to_string())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}