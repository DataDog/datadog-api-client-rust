// Generate campaign report returns "Accepted" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_service_scorecards::ServiceScorecardsAPI;
use datadog_api_client::datadogV2::model::GenerateCampaignReportRequest;
use datadog_api_client::datadogV2::model::GenerateCampaignReportRequestAttributes;
use datadog_api_client::datadogV2::model::GenerateCampaignReportRequestData;
use datadog_api_client::datadogV2::model::GenerateCampaignReportRequestDataType;
use datadog_api_client::datadogV2::model::SlackRoutingOptions;

#[tokio::main]
async fn main() {
    let body = GenerateCampaignReportRequest::new(GenerateCampaignReportRequestData::new(
        GenerateCampaignReportRequestAttributes::new(
            SlackRoutingOptions::new()
                .channel_id("C024BDQ4N".to_string())
                .channel_name("service-scorecards".to_string())
                .workspace_id("T024BDQ4N".to_string())
                .workspace_name("datadog-workspace".to_string()),
        ),
        GenerateCampaignReportRequestDataType::CAMPAIGN_REPORT,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.GenerateScorecardCampaignReport", true);
    let api = ServiceScorecardsAPI::with_config(configuration);
    let resp = api
        .generate_scorecard_campaign_report("c10ODp0VCrrIpXmz".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
