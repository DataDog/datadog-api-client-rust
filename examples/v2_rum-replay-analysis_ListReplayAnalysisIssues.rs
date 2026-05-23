// List replay analysis issues returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_replay_analysis::ListReplayAnalysisIssuesOptionalParams;
use datadog_api_client::datadogV2::api_rum_replay_analysis::RumReplayAnalysisAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ListReplayAnalysisIssues", true);
    let api = RumReplayAnalysisAPI::with_config(configuration);
    let resp = api
        .list_replay_analysis_issues(ListReplayAnalysisIssuesOptionalParams::default())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
