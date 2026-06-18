// Reorder due date rules returns "Successfully reordered the due date rules"
// response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::DueDateRuleReorderItem;
use datadog_api_client::datadogV2::model::DueDateRuleReorderRequest;
use datadog_api_client::datadogV2::model::DueDateRuleType;

#[tokio::main]
async fn main() {
    // there is a valid "valid_due_date_rule" in the system
    let valid_due_date_rule_data_id =
        uuid::Uuid::parse_str(&std::env::var("VALID_DUE_DATE_RULE_DATA_ID").unwrap())
            .expect("Invalid UUID");
    let body = DueDateRuleReorderRequest::new(vec![DueDateRuleReorderItem::new(
        valid_due_date_rule_data_id.clone(),
        DueDateRuleType::DUE_DATE_RULES,
    )]);
    let mut configuration = datadog::Configuration::new();
    configuration
        .set_unstable_operation_enabled("v2.ReorderSecurityFindingsAutomationDueDateRules", true);
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api
        .reorder_security_findings_automation_due_date_rules(body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
