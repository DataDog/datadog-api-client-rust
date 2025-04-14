// List active tags returns "Success" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_metrics::ListActiveMetricConfigurationsOptionalParams;
use datadog_api_client::datadogV2::api_metrics::MetricsAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = MetricsAPI::with_config(configuration);
    let resp = api
        .list_active_metric_configurations(
            "metric_name".to_string(),
            ListActiveMetricConfigurationsOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
