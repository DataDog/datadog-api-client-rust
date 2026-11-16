// Get the impact of an error tracking issue returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_error_tracking::ErrorTrackingAPI;

#[tokio::main]
async fn main() {
    // there is a valid "issue" in the system
    let issue_id = std::env::var("ISSUE_ID").unwrap();
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.get_issue_impact", true);
    let api = ErrorTrackingAPI::with_config(configuration);
    let resp = api
        .get_issue_impact(issue_id.clone(), 1635333071000, 1636629071000)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
