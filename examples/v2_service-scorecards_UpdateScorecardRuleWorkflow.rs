// Associate workflow with rule returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_service_scorecards::ServiceScorecardsAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdateScorecardRuleWorkflow", true);
    let api = ServiceScorecardsAPI::with_config(configuration);
    let resp = api
        .update_scorecard_rule_workflow(
            "rule_id".to_string(),
            "550e8400-e29b-41d4-a716-446655440000".to_string(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
