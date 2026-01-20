// List Entity Risk Scores returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_entity_risk_scores::EntityRiskScoresAPI;
use datadog_api_client::datadogV2::api_entity_risk_scores::ListEntityRiskScoresOptionalParams;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ListEntityRiskScores", true);
    let api = EntityRiskScoresAPI::with_config(configuration);
    let resp = api
        .list_entity_risk_scores(ListEntityRiskScoresOptionalParams::default())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
