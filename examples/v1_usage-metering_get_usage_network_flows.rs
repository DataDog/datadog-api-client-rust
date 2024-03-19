// get hourly usage for network flows returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_usage_metering::GetUsageNetworkFlowsOptionalParams;
use datadog_api_client::datadogV1::api::api_usage_metering::UsageMeteringAPI;

#[tokio::main]
async fn main() {
    let configuration = Configuration::new();
    let api = UsageMeteringAPI::with_config(configuration);
    let resp = api
        .get_usage_network_flows(
            DateTime::parse_from_rfc3339("2021-11-11T11:11:11.111000+00:00")
                .expect("Failed to parse datetime")
                .with_timezone(&Utc),
            GetUsageNetworkFlowsOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
