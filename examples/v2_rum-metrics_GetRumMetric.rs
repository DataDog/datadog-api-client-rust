// Get a rum-based metric returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_metrics::RumMetricsAPI;

#[tokio::main]
async fn main() {
    // there is a valid "rum_metric" in the system
    let rum_metric_data_id = std::env::var("RUM_METRIC_DATA_ID").unwrap();
    let configuration = datadog::Configuration::new();
    let api = RumMetricsAPI::with_config(configuration);
    let resp = api.get_rum_metric(rum_metric_data_id.clone()).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
