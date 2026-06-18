// Create a ticket creation rule returns "Successfully created the ticket creation
// rule" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::AutomationRuleScope;
use datadog_api_client::datadogV2::model::SecurityFindingType;
use datadog_api_client::datadogV2::model::TicketCreationRuleAction;
use datadog_api_client::datadogV2::model::TicketCreationRuleAttributesCreate;
use datadog_api_client::datadogV2::model::TicketCreationRuleCreateRequest;
use datadog_api_client::datadogV2::model::TicketCreationRuleDataCreate;
use datadog_api_client::datadogV2::model::TicketCreationRuleType;
use datadog_api_client::datadogV2::model::TicketCreationTarget;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body = TicketCreationRuleCreateRequest::new(TicketCreationRuleDataCreate::new(
        TicketCreationRuleAttributesCreate::new(
            TicketCreationRuleAction::new(
                10,
                Uuid::parse_str("11111111-1111-1111-1111-111111111111").expect("invalid UUID"),
                TicketCreationTarget::JIRA,
            ),
            "Example-Security-Monitoring".to_string(),
            AutomationRuleScope::new(vec![SecurityFindingType::MISCONFIGURATION])
                .query("env:staging".to_string()),
        )
        .enabled(true),
        TicketCreationRuleType::TICKET_CREATION_RULES,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled(
        "v2.CreateSecurityFindingsAutomationTicketCreationRule",
        true,
    );
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api
        .create_security_findings_automation_ticket_creation_rule(body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
