// Get hourly usage for application security returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_usage_metering::*;

#[tokio::main]
async fn main() {
    let configuration = Configuration::new();
    let api = UsageMeteringAPI::with_config(configuration);
    let resp = api
        .get_usage_application_security_monitoring(
            "2021-11-06T11:11:11+00:00".to_string(),
            GetUsageApplicationSecurityMonitoringOptionalParams::default()
                .end_hr("2021-11-08T11:11:11+00:00".to_string()),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}