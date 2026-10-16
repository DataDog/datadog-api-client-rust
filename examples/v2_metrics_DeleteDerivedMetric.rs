// Delete a derived metric returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_metrics::MetricsAPI;

#[tokio::main]
async fn main() {
    // there is a valid "derived_metric" in the system
    let derived_metric_data_id = std::env::var("DERIVED_METRIC_DATA_ID").unwrap();
    let configuration = datadog::Configuration::new();
    let api = MetricsAPI::with_config(configuration);
    let resp = api
        .delete_derived_metric(derived_metric_data_id.clone())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
