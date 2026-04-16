// Get an indicator of compromise returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.GetIndicatorOfCompromise", true);
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api
        .get_indicator_of_compromise(
            "masscan/1.3 (https://github.com/robertdavidgraham/masscan)".to_string(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
