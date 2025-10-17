// Create a case type returns "CREATED" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_case_management_type::CaseManagementTypeAPI;
use datadog_api_client::datadogV2::model::CaseTypeCreate;
use datadog_api_client::datadogV2::model::CaseTypeCreateRequest;
use datadog_api_client::datadogV2::model::CaseTypeResourceAttributes;
use datadog_api_client::datadogV2::model::CaseTypeResourceType;

#[tokio::main]
async fn main() {
    let body = CaseTypeCreateRequest::new(CaseTypeCreate::new(
        CaseTypeResourceAttributes::new("Investigation".to_string())
            .description("Investigations done in case management".to_string())
            .emoji("👑".to_string()),
        CaseTypeResourceType::CASE_TYPE,
    ));
    let configuration = datadog::Configuration::new();
    let api = CaseManagementTypeAPI::with_config(configuration);
    let resp = api.create_case_type(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
