// Reorder ticket creation rules returns "Successfully reordered the ticket
// creation rules" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::TicketCreationRuleReorderItem;
use datadog_api_client::datadogV2::model::TicketCreationRuleReorderRequest;
use datadog_api_client::datadogV2::model::TicketCreationRuleType;

#[tokio::main]
async fn main() {
    // there is a valid "valid_ticket_creation_rule" in the system
    let valid_ticket_creation_rule_data_id =
        uuid::Uuid::parse_str(&std::env::var("VALID_TICKET_CREATION_RULE_DATA_ID").unwrap())
            .expect("Invalid UUID");
    let body = TicketCreationRuleReorderRequest::new(vec![TicketCreationRuleReorderItem::new(
        valid_ticket_creation_rule_data_id.clone(),
        TicketCreationRuleType::TICKET_CREATION_RULES,
    )]);
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled(
        "v2.ReorderSecurityFindingsAutomationTicketCreationRules",
        true,
    );
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api
        .reorder_security_findings_automation_ticket_creation_rules(body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
