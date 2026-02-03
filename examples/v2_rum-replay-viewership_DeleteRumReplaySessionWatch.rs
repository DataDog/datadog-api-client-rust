// Delete rum replay session watch returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_replay_viewership::RumReplayViewershipAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = RumReplayViewershipAPI::with_config(configuration);
    let resp = api
        .delete_rum_replay_session_watch("00000000-0000-0000-0000-000000000001".to_string())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
