// Create a case view returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_case_management::CaseManagementAPI;
use datadog_api_client::datadogV2::model::CaseViewCreate;
use datadog_api_client::datadogV2::model::CaseViewCreateAttributes;
use datadog_api_client::datadogV2::model::CaseViewCreateRequest;
use datadog_api_client::datadogV2::model::CaseViewResourceType;

#[tokio::main]
async fn main() {
    let body = CaseViewCreateRequest::new(CaseViewCreate::new(
        CaseViewCreateAttributes::new(
            "Open bugs".to_string(),
            "e555e290-ed65-49bd-ae18-8acbfcf18db7".to_string(),
            "status:open type:bug".to_string(),
        ),
        CaseViewResourceType::VIEW,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateCaseView", true);
    let api = CaseManagementAPI::with_config(configuration);
    let resp = api.create_case_view(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
