// Create investigation notebook for case returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_case_management::CaseManagementAPI;
use datadog_api_client::datadogV2::model::CaseInvestigationNotebookCreateData;
use datadog_api_client::datadogV2::model::CaseInvestigationNotebookCreateRequest;
use datadog_api_client::datadogV2::model::CaseInvestigationNotebookResourceType;

#[tokio::main]
async fn main() {
    let body = CaseInvestigationNotebookCreateRequest::new(
        CaseInvestigationNotebookCreateData::new(CaseInvestigationNotebookResourceType::NOTEBOOK),
    );
    let configuration = datadog::Configuration::new();
    let api = CaseManagementAPI::with_config(configuration);
    let resp = api.create_case_notebook("case_id".to_string(), body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
