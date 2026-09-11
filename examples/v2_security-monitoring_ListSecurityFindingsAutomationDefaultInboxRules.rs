// Get all default inbox rules returns "Successfully retrieved the list of default
// inbox rules" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration
        .set_unstable_operation_enabled("v2.ListSecurityFindingsAutomationDefaultInboxRules", true);
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api
        .list_security_findings_automation_default_inbox_rules()
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
