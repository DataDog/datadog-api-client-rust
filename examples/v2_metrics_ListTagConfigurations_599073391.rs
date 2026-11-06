// List all metrics with volumes and query activity returns "Success" response
// with pagination
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_metrics::ListTagConfigurationsOptionalParams;
use datadog_api_client::datadogV2::api_metrics::MetricsAPI;
use futures_util::pin_mut;
use futures_util::stream::StreamExt;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = MetricsAPI::with_config(configuration);
    let response =
        api.list_tag_configurations_with_pagination(ListTagConfigurationsOptionalParams::default());
    pin_mut!(response);
    while let Some(resp) = response.next().await {
        if let Ok(value) = resp {
            println!("{:#?}", value);
        } else {
            println!("{:#?}", resp.unwrap_err());
        }
    }
}
