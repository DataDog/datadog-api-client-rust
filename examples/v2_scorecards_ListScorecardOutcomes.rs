// List all rule outcomes returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_scorecards::ListScorecardOutcomesOptionalParams;
use datadog_api_client::datadogV2::api_scorecards::ScorecardsAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = ScorecardsAPI::with_config(configuration);
    let resp = api
        .list_scorecard_outcomes(ListScorecardOutcomesOptionalParams::default())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
