// Create ServiceNow ticket for case returns "Accepted" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_case_management::CaseManagementAPI;
use datadog_api_client::datadogV2::model::ServiceNowTicketCreateAttributes;
use datadog_api_client::datadogV2::model::ServiceNowTicketCreateData;
use datadog_api_client::datadogV2::model::ServiceNowTicketCreateRequest;
use datadog_api_client::datadogV2::model::ServiceNowTicketResourceType;

#[tokio::main]
async fn main() {
    let body = ServiceNowTicketCreateRequest::new(ServiceNowTicketCreateData::new(
        ServiceNowTicketCreateAttributes::new("my-instance".to_string())
            .assignment_group("IT Support".to_string()),
        ServiceNowTicketResourceType::TICKETS,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateCaseServiceNowTicket", true);
    let api = CaseManagementAPI::with_config(configuration);
    let resp = api
        .create_case_service_now_ticket("case_id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
