// Bulk remove rum replay playlist sessions returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_replay_playlists::RumReplayPlaylistsAPI;
use datadog_api_client::datadogV2::model::SessionIdArray;
use datadog_api_client::datadogV2::model::SessionIdData;
use datadog_api_client::datadogV2::model::ViewershipHistorySessionDataType;

#[tokio::main]
async fn main() {
    let body = SessionIdArray::new(vec![SessionIdData::new(
        ViewershipHistorySessionDataType::RUM_REPLAY_SESSION,
    )
    .id("00000000-0000-0000-0000-000000000001".to_string())]);
    let configuration = datadog::Configuration::new();
    let api = RumReplayPlaylistsAPI::with_config(configuration);
    let resp = api
        .bulk_remove_rum_replay_playlist_sessions(1234567, body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
