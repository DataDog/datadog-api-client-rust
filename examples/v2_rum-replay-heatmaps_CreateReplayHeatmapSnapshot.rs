// Create replay heatmap snapshot returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_replay_heatmaps::RumReplayHeatmapsAPI;
use datadog_api_client::datadogV2::model::SnapshotCreateRequest;
use datadog_api_client::datadogV2::model::SnapshotCreateRequestData;
use datadog_api_client::datadogV2::model::SnapshotCreateRequestDataAttributes;
use datadog_api_client::datadogV2::model::SnapshotUpdateRequestDataType;

#[tokio::main]
async fn main() {
    let body = SnapshotCreateRequest::new(
        SnapshotCreateRequestData::new(SnapshotUpdateRequestDataType::SNAPSHOTS).attributes(
            SnapshotCreateRequestDataAttributes::new(
                "aaaaaaaa-1111-2222-3333-bbbbbbbbbbbb".to_string(),
                "desktop".to_string(),
                "11111111-2222-3333-4444-555555555555".to_string(),
                false,
                "My Snapshot".to_string(),
                0,
                "/home".to_string(),
            ),
        ),
    );
    let configuration = datadog::Configuration::new();
    let api = RumReplayHeatmapsAPI::with_config(configuration);
    let resp = api.create_replay_heatmap_snapshot(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
