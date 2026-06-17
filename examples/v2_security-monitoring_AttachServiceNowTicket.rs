// Attach security findings to a ServiceNow ticket returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::AttachServiceNowTicketRequest;
use datadog_api_client::datadogV2::model::AttachServiceNowTicketRequestData;
use datadog_api_client::datadogV2::model::AttachServiceNowTicketRequestDataAttributes;
use datadog_api_client::datadogV2::model::AttachServiceNowTicketRequestDataRelationships;
use datadog_api_client::datadogV2::model::CaseManagementProject;
use datadog_api_client::datadogV2::model::CaseManagementProjectData;
use datadog_api_client::datadogV2::model::CaseManagementProjectDataType;
use datadog_api_client::datadogV2::model::FindingData;
use datadog_api_client::datadogV2::model::FindingDataType;
use datadog_api_client::datadogV2::model::Findings;
use datadog_api_client::datadogV2::model::ServiceNowTicketsDataType;

#[tokio::main]
async fn main() {
    let body =
        AttachServiceNowTicketRequest::new(
            AttachServiceNowTicketRequestData::new(
                AttachServiceNowTicketRequestDataAttributes::new(
                    "https://example.service-now.com/now/nav/ui/classic/params/target/incident.do?sys_id=abcdef0123456789abcdef0123456789".to_string(),
                ),
                AttachServiceNowTicketRequestDataRelationships::new(
                    Findings
                    ::new().data(
                        vec![
                            FindingData::new(
                                "ZGVmLTAwcC1pZXJ-aS0wZjhjNjMyZDNmMzRlZTgzNw==".to_string(),
                                FindingDataType::FINDINGS,
                            )
                        ],
                    ),
                    CaseManagementProject::new(
                        CaseManagementProjectData::new(
                            "aeadc05e-98a8-11ec-ac2c-da7ad0900001".to_string(),
                            CaseManagementProjectDataType::PROJECTS,
                        ),
                    ),
                ),
                ServiceNowTicketsDataType::SERVICENOW_TICKETS,
            ),
        );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.AttachServiceNowTicket", true);
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api.attach_service_now_ticket(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
