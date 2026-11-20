// List metadata for multiple metrics returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_metrics::MetricsAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.list_metric_metadata", true);
    let api = MetricsAPI::with_config(configuration);
    let resp =
        api
            .list_metric_metadata(
                "static_test_metric_donotdelete,static_test_metric_donotdelete,not.a.real.metric.examplemetric".to_string(),
            )
            .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
