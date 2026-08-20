// Reorder severity modifier rules returns "Successfully reordered the severity
// modifier rules" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::SeverityModifierRuleReorderItem;
use datadog_api_client::datadogV2::model::SeverityModifierRuleReorderRequest;
use datadog_api_client::datadogV2::model::SeverityModifierRuleType;

#[tokio::main]
async fn main() {
    // there is a valid "valid_severity_modifier_rule" in the system
    let valid_severity_modifier_rule_data_id =
        uuid::Uuid::parse_str(&std::env::var("VALID_SEVERITY_MODIFIER_RULE_DATA_ID").unwrap())
            .expect("Invalid UUID");
    let body = SeverityModifierRuleReorderRequest::new(vec![SeverityModifierRuleReorderItem::new(
        valid_severity_modifier_rule_data_id.clone(),
        SeverityModifierRuleType::SEVERITY_MODIFIER_RULES,
    )]);
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled(
        "v2.ReorderSecurityFindingsAutomationSeverityModifierRules",
        true,
    );
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api
        .reorder_security_findings_automation_severity_modifier_rules(body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
