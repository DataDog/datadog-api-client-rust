// Generate team-specific campaign reports returns "Accepted" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_service_scorecards::ServiceScorecardsAPI;
use datadog_api_client::datadogV2::model::EntityOwnerDestination;
use datadog_api_client::datadogV2::model::GenerateCampaignTeamReportsRequest;
use datadog_api_client::datadogV2::model::GenerateCampaignTeamReportsRequestAttributes;
use datadog_api_client::datadogV2::model::GenerateCampaignTeamReportsRequestData;
use datadog_api_client::datadogV2::model::GenerateCampaignTeamReportsRequestDataType;
use datadog_api_client::datadogV2::model::SlackRoutingOptions;

#[tokio::main]
async fn main() {
    let body =
        GenerateCampaignTeamReportsRequest::new(GenerateCampaignTeamReportsRequestData::new(
            GenerateCampaignTeamReportsRequestAttributes::new(vec![EntityOwnerDestination::new(
                SlackRoutingOptions::new()
                    .channel_id("C024BDQ4N".to_string())
                    .workspace_id("T024BDQ4N".to_string()),
                "550e8400-e29b-41d4-a716-446655440000".to_string(),
            )]),
            GenerateCampaignTeamReportsRequestDataType::CAMPAIGN_TEAM_REPORT,
        ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.GenerateScorecardCampaignTeamReports", true);
    let api = ServiceScorecardsAPI::with_config(configuration);
    let resp = api
        .generate_scorecard_campaign_team_reports("c10ODp0VCrrIpXmz".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
