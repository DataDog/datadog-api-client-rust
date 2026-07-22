// Update case comment returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_case_management::CaseManagementAPI;
use datadog_api_client::datadogV2::model::CaseResourceType;
use datadog_api_client::datadogV2::model::CaseUpdateComment;
use datadog_api_client::datadogV2::model::CaseUpdateCommentAttributes;
use datadog_api_client::datadogV2::model::CaseUpdateCommentRequest;

#[tokio::main]
async fn main() {
    let body = CaseUpdateCommentRequest::new(CaseUpdateComment::new(
        CaseUpdateCommentAttributes::new("Updated comment text".to_string()),
        CaseResourceType::CASE,
    ));
    let configuration = datadog::Configuration::new();
    let api = CaseManagementAPI::with_config(configuration);
    let resp = api
        .update_case_comment("case_id".to_string(), "cell_id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
