// List all metrics with volumes and query activity for top metrics by indexed
// volume returns "Success" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_metrics::ListTagConfigurationsOptionalParams;
use datadog_api_client::datadogV2::api_metrics::MetricsAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = MetricsAPI::with_config(configuration);
    let resp = api
        .list_tag_configurations(
            ListTagConfigurationsOptionalParams::default()
                .include("metric_volumes".to_string())
                .sort("-metric_volumes.indexed_volume".to_string())
                .page_size(20),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
