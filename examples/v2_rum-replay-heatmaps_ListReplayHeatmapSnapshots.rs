// List replay heatmap snapshots returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_replay_heatmaps::ListReplayHeatmapSnapshotsOptionalParams;
use datadog_api_client::datadogV2::api_rum_replay_heatmaps::RumReplayHeatmapsAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = RumReplayHeatmapsAPI::with_config(configuration);
    let resp = api
        .list_replay_heatmap_snapshots(
            "/home".to_string(),
            ListReplayHeatmapSnapshotsOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
