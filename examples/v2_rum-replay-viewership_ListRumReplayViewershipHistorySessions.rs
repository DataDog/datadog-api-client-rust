// List rum replay viewership history sessions returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_replay_viewership::ListRumReplayViewershipHistorySessionsOptionalParams;
use datadog_api_client::datadogV2::api_rum_replay_viewership::RumReplayViewershipAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = RumReplayViewershipAPI::with_config(configuration);
    let resp = api
        .list_rum_replay_viewership_history_sessions(
            ListRumReplayViewershipHistorySessionsOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
