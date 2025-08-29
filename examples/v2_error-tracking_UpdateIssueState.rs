// Update the state of an issue returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_error_tracking::ErrorTrackingAPI;
use datadog_api_client::datadogV2::model::IssueState;
use datadog_api_client::datadogV2::model::IssueUpdateStateRequest;
use datadog_api_client::datadogV2::model::IssueUpdateStateRequestData;
use datadog_api_client::datadogV2::model::IssueUpdateStateRequestDataAttributes;
use datadog_api_client::datadogV2::model::IssueUpdateStateRequestDataType;

#[tokio::main]
async fn main() {
    // there is a valid "issue" in the system
    let issue_id = std::env::var("ISSUE_ID").unwrap();
    let body = IssueUpdateStateRequest::new(IssueUpdateStateRequestData::new(
        IssueUpdateStateRequestDataAttributes::new(IssueState::RESOLVED),
        issue_id.clone(),
        IssueUpdateStateRequestDataType::ERROR_TRACKING_ISSUE,
    ));
    let configuration = datadog::Configuration::new();
    let api = ErrorTrackingAPI::with_config(configuration);
    let resp = api.update_issue_state(issue_id.clone(), body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
