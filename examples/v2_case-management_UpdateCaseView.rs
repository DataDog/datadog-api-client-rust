// Update a case view returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_case_management::CaseManagementAPI;
use datadog_api_client::datadogV2::model::CaseViewResourceType;
use datadog_api_client::datadogV2::model::CaseViewUpdate;
use datadog_api_client::datadogV2::model::CaseViewUpdateAttributes;
use datadog_api_client::datadogV2::model::CaseViewUpdateRequest;

#[tokio::main]
async fn main() {
    let body = CaseViewUpdateRequest::new(
        CaseViewUpdate::new(CaseViewResourceType::VIEW).attributes(CaseViewUpdateAttributes::new()),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdateCaseView", true);
    let api = CaseManagementAPI::with_config(configuration);
    let resp = api.update_case_view("view_id".to_string(), body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
