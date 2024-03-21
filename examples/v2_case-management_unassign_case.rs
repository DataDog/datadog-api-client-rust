// Unassign case returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_case_management::CaseManagementAPI;
use datadog_api_client::datadogV2::model::CaseEmpty;
use datadog_api_client::datadogV2::model::CaseEmptyRequest;
use datadog_api_client::datadogV2::model::CaseResourceType;

#[tokio::main]
async fn main() {
    // there is a valid "case" in the system
    let case_id = std::env::var("CASE_ID").unwrap();
    let body = CaseEmptyRequest::new(CaseEmpty::new(CaseResourceType::CASE));
    let configuration = Configuration::new();
    let api = CaseManagementAPI::with_config(configuration);
    let resp = api.unassign_case(case_id.clone(), body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
