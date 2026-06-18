// Reorder mute rules returns "Successfully reordered the mute rules" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::MuteRuleReorderItem;
use datadog_api_client::datadogV2::model::MuteRuleReorderRequest;
use datadog_api_client::datadogV2::model::MuteRuleType;

#[tokio::main]
async fn main() {
    // there is a valid "valid_mute_rule" in the system
    let valid_mute_rule_data_id =
        uuid::Uuid::parse_str(&std::env::var("VALID_MUTE_RULE_DATA_ID").unwrap())
            .expect("Invalid UUID");
    let body = MuteRuleReorderRequest::new(vec![MuteRuleReorderItem::new(
        valid_mute_rule_data_id.clone(),
        MuteRuleType::MUTE_RULES,
    )]);
    let mut configuration = datadog::Configuration::new();
    configuration
        .set_unstable_operation_enabled("v2.ReorderSecurityFindingsAutomationMuteRules", true);
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api
        .reorder_security_findings_automation_mute_rules(body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
