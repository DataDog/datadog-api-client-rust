// Add rum replay session to playlist returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_replay_playlists::AddRumReplaySessionToPlaylistOptionalParams;
use datadog_api_client::datadogV2::api_rum_replay_playlists::RumReplayPlaylistsAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = RumReplayPlaylistsAPI::with_config(configuration);
    let resp = api
        .add_rum_replay_session_to_playlist(
            1704067200000,
            1234567,
            "00000000-0000-0000-0000-000000000001".to_string(),
            AddRumReplaySessionToPlaylistOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
