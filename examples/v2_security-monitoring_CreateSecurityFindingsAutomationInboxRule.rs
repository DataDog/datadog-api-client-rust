// Create an inbox rule returns "Successfully created the inbox rule" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::AutomationRuleScope;
use datadog_api_client::datadogV2::model::InboxRuleAction;
use datadog_api_client::datadogV2::model::InboxRuleAttributesCreate;
use datadog_api_client::datadogV2::model::InboxRuleCreateRequest;
use datadog_api_client::datadogV2::model::InboxRuleDataCreate;
use datadog_api_client::datadogV2::model::InboxRuleType;
use datadog_api_client::datadogV2::model::SecurityFindingType;

#[tokio::main]
async fn main() {
    let body = InboxRuleCreateRequest::new(InboxRuleDataCreate::new(
        InboxRuleAttributesCreate::new(
            InboxRuleAction::new().description("Needs triage".to_string()),
            "Example-Security-Monitoring".to_string(),
            AutomationRuleScope::new(vec![SecurityFindingType::MISCONFIGURATION])
                .query("env:staging".to_string()),
        )
        .enabled(true),
        InboxRuleType::INBOX_RULES,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration
        .set_unstable_operation_enabled("v2.CreateSecurityFindingsAutomationInboxRule", true);
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api
        .create_security_findings_automation_inbox_rule(body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
