// Create a case link returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_case_management::CaseManagementAPI;
use datadog_api_client::datadogV2::model::CaseLinkAttributes;
use datadog_api_client::datadogV2::model::CaseLinkCreate;
use datadog_api_client::datadogV2::model::CaseLinkCreateRequest;
use datadog_api_client::datadogV2::model::CaseLinkResourceType;

#[tokio::main]
async fn main() {
    let body = CaseLinkCreateRequest::new(CaseLinkCreate::new(
        CaseLinkAttributes::new(
            "4417921d-0866-4a38-822c-6f2a0f65f77d".to_string(),
            "CASE".to_string(),
            "bf0cbac6-4c16-4cfb-b6bf-ca5e0ec37a4f".to_string(),
            "CASE".to_string(),
            "BLOCKS".to_string(),
        ),
        CaseLinkResourceType::LINK,
    ));
    let configuration = datadog::Configuration::new();
    let api = CaseManagementAPI::with_config(configuration);
    let resp = api.create_case_link(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
