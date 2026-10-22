// Get the details of an error tracking issue with samples returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_error_tracking::ErrorTrackingAPI;
use datadog_api_client::datadogV2::api_error_tracking::GetIssueOptionalParams;
use datadog_api_client::datadogV2::model::GetIssueIncludeQueryParameterItem;
use datadog_api_client::datadogV2::model::IssueSampleBias;

#[tokio::main]
async fn main() {
    // there is a valid "issue" in the system
    let issue_id = std::env::var("ISSUE_ID").unwrap();
    let configuration = datadog::Configuration::new();
    let api = ErrorTrackingAPI::with_config(configuration);
    let resp = api
        .get_issue(
            issue_id.clone(),
            GetIssueOptionalParams::default()
                .include(vec![GetIssueIncludeQueryParameterItem::SAMPLES])
                .filter_samples_biases(vec![IssueSampleBias::LATEST]),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
