// Update case due date returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_case_management::CaseManagementAPI;
use datadog_api_client::datadogV2::model::CaseResourceType;
use datadog_api_client::datadogV2::model::CaseUpdateDueDate;
use datadog_api_client::datadogV2::model::CaseUpdateDueDateAttributes;
use datadog_api_client::datadogV2::model::CaseUpdateDueDateRequest;

#[tokio::main]
async fn main() {
    let body = CaseUpdateDueDateRequest::new(CaseUpdateDueDate::new(
        CaseUpdateDueDateAttributes::new("2026-12-31".to_string()),
        CaseResourceType::CASE,
    ));
    let configuration = datadog::Configuration::new();
    let api = CaseManagementAPI::with_config(configuration);
    let resp = api.update_case_due_date("case_id".to_string(), body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
