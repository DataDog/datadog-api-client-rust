// Get case timeline returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_case_management::CaseManagementAPI;
use datadog_api_client::datadogV2::api_case_management::ListCaseTimelineOptionalParams;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ListCaseTimeline", true);
    let api = CaseManagementAPI::with_config(configuration);
    let resp = api
        .list_case_timeline(
            "case_id".to_string(),
            ListCaseTimelineOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
