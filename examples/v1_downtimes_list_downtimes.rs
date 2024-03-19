// Get all downtimes returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_downtimes::DowntimesAPI;
use datadog_api_client::datadogV1::api::api_downtimes::ListDowntimesOptionalParams;

#[tokio::main]
async fn main() {
    let configuration = Configuration::new();
    let api = DowntimesAPI::with_config(configuration);
    let resp = api
        .list_downtimes(ListDowntimesOptionalParams::default().with_creator(true))
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
