// Get a campaign returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_service_scorecards::GetScorecardCampaignOptionalParams;
use datadog_api_client::datadogV2::api_service_scorecards::ServiceScorecardsAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.GetScorecardCampaign", true);
    let api = ServiceScorecardsAPI::with_config(configuration);
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
