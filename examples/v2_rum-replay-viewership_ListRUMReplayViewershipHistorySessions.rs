// List RUM replay viewership history sessions returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_replay_viewership::ListRUMReplayViewershipHistorySessionsOptionalParams;
use datadog_api_client::datadogV2::api_rum_replay_viewership::RUMReplayViewershipAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = RUMReplayViewershipAPI::with_config(configuration);
    let resp = api
        .list_rum_replay_viewership_history_sessions(
            ListRUMReplayViewershipHistorySessionsOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
