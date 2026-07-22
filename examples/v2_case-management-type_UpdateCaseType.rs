// Update a case type returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_case_management_type::CaseManagementTypeAPI;
use datadog_api_client::datadogV2::model::CaseTypeResourceAttributes;
use datadog_api_client::datadogV2::model::CaseTypeResourceType;
use datadog_api_client::datadogV2::model::CaseTypeUpdate;
use datadog_api_client::datadogV2::model::CaseTypeUpdateRequest;

#[tokio::main]
async fn main() {
    let body = CaseTypeUpdateRequest::new(
        CaseTypeUpdate::new(CaseTypeResourceType::CASE_TYPE).attributes(
            CaseTypeResourceAttributes::new("Investigation".to_string())
                .description("Investigations done in case management".to_string())
                .emoji("🕵🏻‍♂️".to_string()),
        ),
    );
    let configuration = datadog::Configuration::new();
    let api = CaseManagementTypeAPI::with_config(configuration);
    let resp = api.update_case_type("case_type_id".to_string(), body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
