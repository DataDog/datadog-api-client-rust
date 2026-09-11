// Enable a default inbox rule returns "Successfully enabled the default inbox
// rule" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled(
        "v2.EnableSecurityFindingsAutomationDefaultInboxRule",
        true,
    );
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api
        .enable_security_findings_automation_default_inbox_rule("secret_default_rule".to_string())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
