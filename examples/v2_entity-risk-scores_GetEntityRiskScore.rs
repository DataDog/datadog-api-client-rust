// Get Entity Risk Score returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_entity_risk_scores::EntityRiskScoresAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.GetEntityRiskScore", true);
    let api = EntityRiskScoresAPI::with_config(configuration);
    let resp = api
        .get_entity_risk_score("arn:aws:iam::123456789012:user/john.doe".to_string())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
