// Get a campaign returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_scorecards::GetScorecardCampaignOptionalParams;
use datadog_api_client::datadogV2::api_scorecards::ScorecardsAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = ScorecardsAPI::with_config(configuration);
    let resp = api
        .get_scorecard_campaign(
            "c10ODp0VCrrIpXmz".to_string(),
            GetScorecardCampaignOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
