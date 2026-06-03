// Get the RUM configuration returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_config::RumConfigAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.GetRumConfig", true);
    let api = RumConfigAPI::with_config(configuration);
    let resp = api.get_rum_config().await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
