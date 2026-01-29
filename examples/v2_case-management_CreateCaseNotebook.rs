// Create investigation notebook for case returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_case_management::CaseManagementAPI;
use datadog_api_client::datadogV2::model::NotebookCreateData;
use datadog_api_client::datadogV2::model::NotebookCreateRequest;
use datadog_api_client::datadogV2::model::NotebookResourceType;

#[tokio::main]
async fn main() {
    let body = NotebookCreateRequest::new(NotebookCreateData::new(NotebookResourceType::NOTEBOOK));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateCaseNotebook", true);
    let api = CaseManagementAPI::with_config(configuration);
    let resp = api.create_case_notebook("case_id".to_string(), body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
