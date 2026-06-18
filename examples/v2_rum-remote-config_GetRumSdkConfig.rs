// Get a RUM SDK configuration returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_remote_config::RUMRemoteConfigAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.GetRumSdkConfig", true);
    let api = RUMRemoteConfigAPI::with_config(configuration);
    let resp = api.get_rum_sdk_config("config_id".to_string()).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
