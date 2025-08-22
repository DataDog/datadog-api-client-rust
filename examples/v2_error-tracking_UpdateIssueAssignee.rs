// Update the assignee of an issue returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_error_tracking::ErrorTrackingAPI;
use datadog_api_client::datadogV2::model::IssueUpdateAssigneeRequest;
use datadog_api_client::datadogV2::model::IssueUpdateAssigneeRequestData;
use datadog_api_client::datadogV2::model::IssueUpdateAssigneeRequestDataType;

#[tokio::main]
async fn main() {
    // there is a valid "issue" in the system
    let issue_id = std::env::var("ISSUE_ID").unwrap();
    let body = IssueUpdateAssigneeRequest::new(IssueUpdateAssigneeRequestData::new(
        "87cb11a0-278c-440a-99fe-701223c80296".to_string(),
        IssueUpdateAssigneeRequestDataType::ASSIGNEE,
    ));
    let configuration = datadog::Configuration::new();
    let api = ErrorTrackingAPI::with_config(configuration);
    let resp = api.update_issue_assignee(issue_id.clone(), body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
