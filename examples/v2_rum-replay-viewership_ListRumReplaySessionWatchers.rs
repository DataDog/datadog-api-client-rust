// List rum replay session watchers returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_replay_viewership::ListRumReplaySessionWatchersOptionalParams;
use datadog_api_client::datadogV2::api_rum_replay_viewership::RumReplayViewershipAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = RumReplayViewershipAPI::with_config(configuration);
    let resp = api
        .list_rum_replay_session_watchers(
            "00000000-0000-0000-0000-000000000001".to_string(),
            ListRumReplaySessionWatchersOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
