// Create a custom framework returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::CreateCustomFrameworkRequest;
use datadog_api_client::datadogV2::model::FrameworkControl;
use datadog_api_client::datadogV2::model::FrameworkRequirement;

#[tokio::main]
async fn main() {
    let body = CreateCustomFrameworkRequest::new(
        "".to_string(),
        "".to_string(),
        vec![FrameworkRequirement::new(
            vec![FrameworkControl::new("".to_string(), vec!["".to_string()])],
            "".to_string(),
        )],
        "".to_string(),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateCustomFramework", true);
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api.create_custom_framework(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
