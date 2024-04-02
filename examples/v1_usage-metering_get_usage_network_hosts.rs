// Get hourly usage for network hosts returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV1::api::api_usage_metering::GetUsageNetworkHostsOptionalParams;
use datadog_api_client::datadogV1::api::api_usage_metering::UsageMeteringAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = UsageMeteringAPI::with_config(configuration);
    let resp = api
        .get_usage_network_hosts(
            "2021-11-11T11:11:11.111000+00:00".to_string(),
            GetUsageNetworkHostsOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
