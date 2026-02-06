// List all scorecards returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_service_scorecards::ListScorecardsOptionalParams;
use datadog_api_client::datadogV2::api_service_scorecards::ServiceScorecardsAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ListScorecards", true);
    let api = ServiceScorecardsAPI::with_config(configuration);
    let resp = api
        .list_scorecards(ListScorecardsOptionalParams::default())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
