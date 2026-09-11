// Update an inbox rule returns "Successfully updated the inbox rule" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::AutomationRuleScope;
use datadog_api_client::datadogV2::model::InboxRuleAction;
use datadog_api_client::datadogV2::model::InboxRuleAttributesCreate;
use datadog_api_client::datadogV2::model::InboxRuleDataUpdate;
use datadog_api_client::datadogV2::model::InboxRuleType;
use datadog_api_client::datadogV2::model::InboxRuleUpdateRequest;
use datadog_api_client::datadogV2::model::SecurityFindingType;

#[tokio::main]
async fn main() {
    // there is a valid "valid_inbox_rule" in the system
    let valid_inbox_rule_data_id =
        uuid::Uuid::parse_str(&std::env::var("VALID_INBOX_RULE_DATA_ID").unwrap())
            .expect("Invalid UUID");
    let body = InboxRuleUpdateRequest::new(InboxRuleDataUpdate::new(
        InboxRuleAttributesCreate::new(
            InboxRuleAction::new().description("Needs triage".to_string()),
            "Example-Security-Monitoring".to_string(),
            AutomationRuleScope::new(vec![SecurityFindingType::MISCONFIGURATION])
                .query("env:staging".to_string()),
        )
        .enabled(false),
        valid_inbox_rule_data_id.clone(),
        InboxRuleType::INBOX_RULES,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration
        .set_unstable_operation_enabled("v2.UpdateSecurityFindingsAutomationInboxRule", true);
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api
        .update_security_findings_automation_inbox_rule(valid_inbox_rule_data_id.clone(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
