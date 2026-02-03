// Update replay heatmap snapshot returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_replay_heatmaps::RumReplayHeatmapsAPI;
use datadog_api_client::datadogV2::model::SnapshotUpdateRequest;
use datadog_api_client::datadogV2::model::SnapshotUpdateRequestData;
use datadog_api_client::datadogV2::model::SnapshotUpdateRequestDataAttributes;
use datadog_api_client::datadogV2::model::SnapshotUpdateRequestDataType;

#[tokio::main]
async fn main() {
    let body = SnapshotUpdateRequest::new(
        SnapshotUpdateRequestData::new(SnapshotUpdateRequestDataType::SNAPSHOTS)
            .attributes(SnapshotUpdateRequestDataAttributes::new(
                "11111111-2222-3333-4444-555555555555".to_string(),
                false,
                0,
            ))
            .id("00000000-0000-0000-0000-000000000001".to_string()),
    );
    let configuration = datadog::Configuration::new();
    let api = RumReplayHeatmapsAPI::with_config(configuration);
    let resp = api
        .update_replay_heatmap_snapshot("00000000-0000-0000-0000-000000000001".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
