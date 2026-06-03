// Create rum replay session watch returns "Created" response
use chrono::{DateTime, Utc};
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_replay_viewership::RumReplayViewershipAPI;
use datadog_api_client::datadogV2::model::Watch;
use datadog_api_client::datadogV2::model::WatchData;
use datadog_api_client::datadogV2::model::WatchDataAttributes;
use datadog_api_client::datadogV2::model::WatchDataType;

#[tokio::main]
async fn main() {
    let body = Watch::new(
        WatchData::new(WatchDataType::RUM_REPLAY_WATCH).attributes(WatchDataAttributes::new(
            "aaaaaaaa-1111-2222-3333-bbbbbbbbbbbb".to_string(),
            "11111111-2222-3333-4444-555555555555".to_string(),
            DateTime::parse_from_rfc3339("2026-01-13T17:15:53.208340+00:00")
                .expect("Failed to parse datetime")
                .with_timezone(&Utc),
        )),
    );
    let configuration = datadog::Configuration::new();
    let api = RumReplayViewershipAPI::with_config(configuration);
    let resp = api
        .create_rum_replay_session_watch("00000000-0000-0000-0000-000000000001".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
