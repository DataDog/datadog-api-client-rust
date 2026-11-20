// List RUM replay session watchers returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_replay_viewership::ListRUMReplaySessionWatchersOptionalParams;
use datadog_api_client::datadogV2::api_rum_replay_viewership::RUMReplayViewershipAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = RUMReplayViewershipAPI::with_config(configuration);
    let resp = api
        .list_rum_replay_session_watchers(
            "00000000-0000-0000-0000-000000000001".to_string(),
            ListRUMReplaySessionWatchersOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
