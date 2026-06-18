// Update a due date rule returns "Successfully updated the due date rule" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::AutomationRuleScope;
use datadog_api_client::datadogV2::model::DueDateFrom;
use datadog_api_client::datadogV2::model::DueDatePerSeverityItem;
use datadog_api_client::datadogV2::model::DueDateRuleAction;
use datadog_api_client::datadogV2::model::DueDateRuleAttributesCreate;
use datadog_api_client::datadogV2::model::DueDateRuleDataCreate;
use datadog_api_client::datadogV2::model::DueDateRuleType;
use datadog_api_client::datadogV2::model::DueDateRuleUpdateRequest;
use datadog_api_client::datadogV2::model::DueDateSeverity;
use datadog_api_client::datadogV2::model::SecurityFindingType;

#[tokio::main]
async fn main() {
    // there is a valid "valid_due_date_rule" in the system
    let valid_due_date_rule_data_id =
        uuid::Uuid::parse_str(&std::env::var("VALID_DUE_DATE_RULE_DATA_ID").unwrap())
            .expect("Invalid UUID");
    let body = DueDateRuleUpdateRequest::new(DueDateRuleDataCreate::new(
        DueDateRuleAttributesCreate::new(
            DueDateRuleAction::new(
                vec![DueDatePerSeverityItem::new(14, DueDateSeverity::CRITICAL)],
                DueDateFrom::FIRST_SEEN,
            ),
            "Example-Security-Monitoring".to_string(),
            AutomationRuleScope::new(vec![SecurityFindingType::MISCONFIGURATION])
                .query("env:staging".to_string()),
        )
        .enabled(false),
        DueDateRuleType::DUE_DATE_RULES,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration
        .set_unstable_operation_enabled("v2.UpdateSecurityFindingsAutomationDueDateRule", true);
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api
        .update_security_findings_automation_due_date_rule(
            valid_due_date_rule_data_id.clone(),
            body,
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
