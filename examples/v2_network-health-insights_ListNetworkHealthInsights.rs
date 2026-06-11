// List network health insights returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_network_health_insights::ListNetworkHealthInsightsOptionalParams;
use datadog_api_client::datadogV2::api_network_health_insights::NetworkHealthInsightsAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ListNetworkHealthInsights", true);
    let api = NetworkHealthInsightsAPI::with_config(configuration);
    let resp = api
        .list_network_health_insights(ListNetworkHealthInsightsOptionalParams::default())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
