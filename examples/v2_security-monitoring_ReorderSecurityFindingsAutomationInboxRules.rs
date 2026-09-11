// Reorder inbox rules returns "Successfully reordered the inbox rules" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::InboxRuleReorderItem;
use datadog_api_client::datadogV2::model::InboxRuleReorderRequest;
use datadog_api_client::datadogV2::model::InboxRuleType;

#[tokio::main]
async fn main() {
    // there is a valid "valid_inbox_rule" in the system
    let valid_inbox_rule_data_id =
        uuid::Uuid::parse_str(&std::env::var("VALID_INBOX_RULE_DATA_ID").unwrap())
            .expect("Invalid UUID");
    let body = InboxRuleReorderRequest::new(vec![InboxRuleReorderItem::new(
        valid_inbox_rule_data_id.clone(),
        InboxRuleType::INBOX_RULES,
    )]);
    let mut configuration = datadog::Configuration::new();
    configuration
        .set_unstable_operation_enabled("v2.ReorderSecurityFindingsAutomationInboxRules", true);
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api
        .reorder_security_findings_automation_inbox_rules(body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
