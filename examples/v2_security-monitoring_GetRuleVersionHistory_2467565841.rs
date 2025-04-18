// Get rule version history returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::GetRuleVersionHistoryOptionalParams;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;

#[tokio::main]
async fn main() {
    // there is a valid "security_rule" in the system
    let security_rule_id = std::env::var("SECURITY_RULE_ID").unwrap();
    let configuration = datadog::Configuration::new();
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api
        .get_rule_version_history(
            security_rule_id.clone(),
            GetRuleVersionHistoryOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
