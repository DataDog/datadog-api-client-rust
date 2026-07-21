// Update case resolved reason returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_case_management::CaseManagementAPI;
use datadog_api_client::datadogV2::model::CaseResourceType;
use datadog_api_client::datadogV2::model::CaseUpdateResolvedReason;
use datadog_api_client::datadogV2::model::CaseUpdateResolvedReasonAttributes;
use datadog_api_client::datadogV2::model::CaseUpdateResolvedReasonRequest;

#[tokio::main]
async fn main() {
    let body = CaseUpdateResolvedReasonRequest::new(CaseUpdateResolvedReason::new(
        CaseUpdateResolvedReasonAttributes::new("FALSE_POSITIVE".to_string()),
        CaseResourceType::CASE,
    ));
    let configuration = datadog::Configuration::new();
    let api = CaseManagementAPI::with_config(configuration);
    let resp = api
        .update_case_resolved_reason("case_id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
