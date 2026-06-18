// Update a ticket creation rule returns "Successfully updated the ticket creation
// rule" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::AutomationRuleScope;
use datadog_api_client::datadogV2::model::SecurityFindingType;
use datadog_api_client::datadogV2::model::TicketCreationRuleAction;
use datadog_api_client::datadogV2::model::TicketCreationRuleAttributesCreate;
use datadog_api_client::datadogV2::model::TicketCreationRuleDataCreate;
use datadog_api_client::datadogV2::model::TicketCreationRuleType;
use datadog_api_client::datadogV2::model::TicketCreationRuleUpdateRequest;
use datadog_api_client::datadogV2::model::TicketCreationTarget;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    // there is a valid "valid_ticket_creation_rule" in the system
    let valid_ticket_creation_rule_data_id =
        uuid::Uuid::parse_str(&std::env::var("VALID_TICKET_CREATION_RULE_DATA_ID").unwrap())
            .expect("Invalid UUID");
    let body = TicketCreationRuleUpdateRequest::new(TicketCreationRuleDataCreate::new(
        TicketCreationRuleAttributesCreate::new(
            TicketCreationRuleAction::new(
                5,
                Uuid::parse_str("11111111-1111-1111-1111-111111111111").expect("invalid UUID"),
                TicketCreationTarget::JIRA,
            ),
            "Example-Security-Monitoring".to_string(),
            AutomationRuleScope::new(vec![SecurityFindingType::MISCONFIGURATION])
                .query("env:staging".to_string()),
        )
        .enabled(false),
        TicketCreationRuleType::TICKET_CREATION_RULES,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled(
        "v2.UpdateSecurityFindingsAutomationTicketCreationRule",
        true,
    );
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api
        .update_security_findings_automation_ticket_creation_rule(
            valid_ticket_creation_rule_data_id.clone(),
            body,
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
