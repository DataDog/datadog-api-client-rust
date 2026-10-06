// Send batched CI job logs returns "Request accepted for processing" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_ci_visibility_logs::CIVisibilityLogsAPI;
use datadog_api_client::datadogV2::api_ci_visibility_logs::SubmitCILogOptionalParams;
use datadog_api_client::datadogV2::model::CILogItem;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let body = vec![
        CILogItem::new(
            "job-456".to_string(),
            "Starting tests".to_string(),
            "3eacb6f3-ff04-4e10-8a9c-46e6d054024a".to_string(),
        )
        .line_number(1)
        .section_name("tests".to_string())
        .status("notice".to_string())
        .additional_properties(BTreeMap::from([])),
        CILogItem::new(
            "job-456".to_string(),
            "Tests passed".to_string(),
            "3eacb6f3-ff04-4e10-8a9c-46e6d054024a".to_string(),
        )
        .line_number(2)
        .additional_properties(BTreeMap::from([])),
    ];
    let configuration = datadog::Configuration::new();
    let api = CIVisibilityLogsAPI::with_config(configuration);
    let resp = api
        .submit_ci_log(body, SubmitCILogOptionalParams::default())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
