// Create rum replay playlist returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_replay_playlists::RumReplayPlaylistsAPI;
use datadog_api_client::datadogV2::model::Playlist;
use datadog_api_client::datadogV2::model::PlaylistData;
use datadog_api_client::datadogV2::model::PlaylistDataAttributes;
use datadog_api_client::datadogV2::model::PlaylistDataAttributesCreatedBy;
use datadog_api_client::datadogV2::model::PlaylistDataType;

#[tokio::main]
async fn main() {
    let body = Playlist::new(
        PlaylistData::new(PlaylistDataType::RUM_REPLAY_PLAYLIST).attributes(
            PlaylistDataAttributes::new("My Playlist".to_string()).created_by(
                PlaylistDataAttributesCreatedBy::new(
                    "john.doe@example.com".to_string(),
                    "00000000-0000-0000-0000-000000000001".to_string(),
                    "00000000-0000-0000-0000-000000000001".to_string(),
                ),
            ),
        ),
    );
    let configuration = datadog::Configuration::new();
    let api = RumReplayPlaylistsAPI::with_config(configuration);
    let resp = api.create_rum_replay_playlist(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
