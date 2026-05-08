// List replay analysis issue sessions returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_replay_analysis::ListReplayAnalysisIssueSessionsOptionalParams;
use datadog_api_client::datadogV2::api_rum_replay_analysis::RumReplayAnalysisAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ListReplayAnalysisIssueSessions", true);
    let api = RumReplayAnalysisAPI::with_config(configuration);
    let resp = api
        .list_replay_analysis_issue_sessions(
            "00000000-0000-0000-0000-000000000001".to_string(),
            ListReplayAnalysisIssueSessionsOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
