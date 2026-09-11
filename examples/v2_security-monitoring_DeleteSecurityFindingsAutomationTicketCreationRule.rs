// Delete a ticket creation rule returns "Successfully deleted the ticket creation
// rule" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled(
        "v2.DeleteSecurityFindingsAutomationTicketCreationRule",
        true,
    );
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api
        .delete_security_findings_automation_ticket_creation_rule(
            Uuid::parse_str("00000000-0000-0000-0000-000000000000").expect("invalid UUID"),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
