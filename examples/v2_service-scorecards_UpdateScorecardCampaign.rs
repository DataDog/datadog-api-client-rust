// Update a campaign returns "OK" response
use chrono::{DateTime, Utc};
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_service_scorecards::ServiceScorecardsAPI;
use datadog_api_client::datadogV2::model::CampaignType;
use datadog_api_client::datadogV2::model::UpdateCampaignRequest;
use datadog_api_client::datadogV2::model::UpdateCampaignRequestAttributes;
use datadog_api_client::datadogV2::model::UpdateCampaignRequestData;

#[tokio::main]
async fn main() {
    let body = UpdateCampaignRequest::new(UpdateCampaignRequestData::new(
        UpdateCampaignRequestAttributes::new(
            "Q1 Security Campaign".to_string(),
            "550e8400-e29b-41d4-a716-446655440000".to_string(),
            vec![
                "q8MQxk8TCqrHnWkx".to_string(),
                "r9NRyl9UDrsIoXly".to_string(),
            ],
            DateTime::parse_from_rfc3339("2024-01-01T00:00:00+00:00")
                .expect("Failed to parse datetime")
                .with_timezone(&Utc),
            "in_progress".to_string(),
        )
        .description("Campaign to improve security posture for Q1 2024.".to_string())
        .due_date(
            DateTime::parse_from_rfc3339("2024-03-31T23:59:59+00:00")
                .expect("Failed to parse datetime")
                .with_timezone(&Utc),
        )
        .entity_scope("kind:service AND team:platform".to_string())
        .guidance("Please ensure all services pass the security requirements.".to_string())
        .key("q1-security-2024".to_string()),
        CampaignType::CAMPAIGN,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdateScorecardCampaign", true);
    let api = ServiceScorecardsAPI::with_config(configuration);
    let resp = api
        .update_scorecard_campaign("c10ODp0VCrrIpXmz".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
