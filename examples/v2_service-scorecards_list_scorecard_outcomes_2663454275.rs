// List all rule outcomes returns "OK" response with pagination
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_service_scorecards::*;

#[tokio::main]
async fn main() {
    let mut configuration = Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ListScorecardOutcomes", true);
    let api = ServiceScorecardsAPI::with_config(configuration);
    let resp = api
        .list_scorecard_outcomes(
            ListScorecardOutcomesOptionalParams::default()
                .page_size(2)
                .fields_outcome("state".to_string())
                .filter_outcome_service_name("my-service".to_string()),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}