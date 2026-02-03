// List rum replay playlists returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_replay_playlists::ListRumReplayPlaylistsOptionalParams;
use datadog_api_client::datadogV2::api_rum_replay_playlists::RumReplayPlaylistsAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = RumReplayPlaylistsAPI::with_config(configuration);
    let resp = api
        .list_rum_replay_playlists(ListRumReplayPlaylistsOptionalParams::default())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
