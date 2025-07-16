// Get all aggregated connections returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_cloud_network_monitoring::CloudNetworkMonitoringAPI;
use datadog_api_client::datadogV2::api_cloud_network_monitoring::GetAggregatedConnectionsOptionalParams;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = CloudNetworkMonitoringAPI::with_config(configuration);
    let resp = api
        .get_aggregated_connections(GetAggregatedConnectionsOptionalParams::default())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
