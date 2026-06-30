// Create ServiceNow tickets for security findings returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::CaseManagementProject;
use datadog_api_client::datadogV2::model::CaseManagementProjectData;
use datadog_api_client::datadogV2::model::CaseManagementProjectDataType;
use datadog_api_client::datadogV2::model::CasePriority;
use datadog_api_client::datadogV2::model::CreateServiceNowTicketRequestArray;
use datadog_api_client::datadogV2::model::CreateServiceNowTicketRequestData;
use datadog_api_client::datadogV2::model::CreateServiceNowTicketRequestDataAttributes;
use datadog_api_client::datadogV2::model::CreateServiceNowTicketRequestDataRelationships;
use datadog_api_client::datadogV2::model::FindingData;
use datadog_api_client::datadogV2::model::FindingDataType;
use datadog_api_client::datadogV2::model::Findings;
use datadog_api_client::datadogV2::model::ServiceNowTicketsDataType;

#[tokio::main]
async fn main() {
    let body =
        CreateServiceNowTicketRequestArray::new(vec![CreateServiceNowTicketRequestData::new(
            CreateServiceNowTicketRequestDataRelationships::new(
                Findings::new().data(vec![FindingData::new(
                    "ZGVmLTAwcC1pZXJ-aS0wZjhjNjMyZDNmMzRlZTgzNw==".to_string(),
                    FindingDataType::FINDINGS,
                )]),
                CaseManagementProject::new(CaseManagementProjectData::new(
                    "aeadc05e-98a8-11ec-ac2c-da7ad0900001".to_string(),
                    CaseManagementProjectDataType::PROJECTS,
                )),
            ),
            ServiceNowTicketsDataType::SERVICENOW_TICKETS,
        )
        .attributes(
            CreateServiceNowTicketRequestDataAttributes::new()
                .assignee_id("f315bdaf-9ee7-4808-a9c1-99c15bf0f4d0".to_string())
                .description("A description of the ServiceNow ticket.".to_string())
                .priority(CasePriority::NOT_DEFINED)
                .title("A title for the ServiceNow ticket.".to_string()),
        )]);
    let configuration = datadog::Configuration::new();
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api.create_service_now_tickets(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
