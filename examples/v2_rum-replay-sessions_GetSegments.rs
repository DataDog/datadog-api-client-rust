// Get segments returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_replay_sessions::GetSegmentsOptionalParams;
use datadog_api_client::datadogV2::api_rum_replay_sessions::RumReplaySessionsAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = RumReplaySessionsAPI::with_config(configuration);
    let resp = api
        .get_segments(
            "00000000-0000-0000-0000-000000000002".to_string(),
            "00000000-0000-0000-0000-000000000001".to_string(),
            GetSegmentsOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
