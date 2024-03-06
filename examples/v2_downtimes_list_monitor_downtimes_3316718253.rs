// Get active downtimes for a monitor returns "OK" response with pagination
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_downtimes::*;

#[tokio::main]
async fn main() {
    let configuration = Configuration::new();
    let api = DowntimesAPI::with_config(configuration);
    let resp = api
        .list_monitor_downtimes(
            9223372036854775807,
            ListMonitorDowntimesOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}