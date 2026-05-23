// Generate replay summary returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_replay::GenerateReplaySummaryOptionalParams;
use datadog_api_client::datadogV2::api_rum_replay::RumReplayAPI;
use datadog_api_client::datadogV2::model::ReplaySummaryDataRequest;
use datadog_api_client::datadogV2::model::ReplaySummaryRequest;
use datadog_api_client::datadogV2::model::ReplaySummaryRequestType;

#[tokio::main]
async fn main() {
    let body = ReplaySummaryRequest::new(ReplaySummaryDataRequest::new(
        ReplaySummaryRequestType::REPLAY_SUMMARY_REQUEST,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.GenerateReplaySummary", true);
    let api = RumReplayAPI::with_config(configuration);
    let resp = api
        .generate_replay_summary(
            "00000000-0000-0000-0000-000000000001".to_string(),
            "rum".to_string(),
            body,
            GenerateReplaySummaryOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
