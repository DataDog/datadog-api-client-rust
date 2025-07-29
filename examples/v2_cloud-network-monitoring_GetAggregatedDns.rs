// Get all aggregated DNS traffic returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_cloud_network_monitoring::CloudNetworkMonitoringAPI;
use datadog_api_client::datadogV2::api_cloud_network_monitoring::GetAggregatedDnsOptionalParams;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.GetAggregatedDns", true);
    let api = CloudNetworkMonitoringAPI::with_config(configuration);
    let resp = api
        .get_aggregated_dns(GetAggregatedDnsOptionalParams::default())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
