// Get tag key cardinality details returns "Success" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_metrics::MetricsAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = MetricsAPI::with_config(configuration);
    let resp = api
        .get_metric_tag_cardinality_details("metric_name".to_string())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
