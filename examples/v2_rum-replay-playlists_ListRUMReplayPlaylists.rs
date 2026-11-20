// List RUM replay playlists returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_replay_playlists::ListRUMReplayPlaylistsOptionalParams;
use datadog_api_client::datadogV2::api_rum_replay_playlists::RUMReplayPlaylistsAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = RUMReplayPlaylistsAPI::with_config(configuration);
    let resp = api
        .list_rum_replay_playlists(ListRUMReplayPlaylistsOptionalParams::default())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
