// Update an existing scorecard rule returns "Rule updated successfully" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_scorecards::ScorecardsAPI;
use datadog_api_client::datadogV2::model::RuleAttributesRequest;
use datadog_api_client::datadogV2::model::RuleType;
use datadog_api_client::datadogV2::model::UpdateRuleRequest;
use datadog_api_client::datadogV2::model::UpdateRuleRequestData;

#[tokio::main]
async fn main() {
    let body = UpdateRuleRequest::new().data(UpdateRuleRequestData::new(
        RuleAttributesRequest::new()
            .enabled(true)
            .level(2)
            .name("Team Defined".to_string())
            .scope_query("kind:service".to_string())
            .scorecard_name("Deployments automated via Deployment Trains".to_string()),
        RuleType::RULE,
    ));
    let configuration = datadog::Configuration::new();
    let api = ScorecardsAPI::with_config(configuration);
    let resp = api.update_scorecard_rule("rule_id".to_string(), body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
