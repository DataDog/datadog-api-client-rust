// Snooze an issue returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_error_tracking::ErrorTrackingAPI;
use datadog_api_client::datadogV2::model::IssueSnoozeRequest;
use datadog_api_client::datadogV2::model::IssueSnoozeRequestData;
use datadog_api_client::datadogV2::model::IssueSnoozeRequestDataAttributes;
use datadog_api_client::datadogV2::model::IssueSnoozeRequestDataType;
use datadog_api_client::datadogV2::model::IssueWatchCondition;
use datadog_api_client::datadogV2::model::IssueWatchConditionKind;

#[tokio::main]
async fn main() {
    // there is a valid "issue" in the system
    let issue_id = std::env::var("ISSUE_ID").unwrap();
    let body = IssueSnoozeRequest::new(IssueSnoozeRequestData::new(
        IssueSnoozeRequestDataAttributes::new(
            IssueWatchCondition::new(IssueWatchConditionKind::ELAPSED_TIME)
                .duration("168h".to_string()),
        ),
        issue_id.clone(),
        IssueSnoozeRequestDataType::ERROR_TRACKING_ISSUE,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.snooze_issue", true);
    let api = ErrorTrackingAPI::with_config(configuration);
    let resp = api.snooze_issue(issue_id.clone(), body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
