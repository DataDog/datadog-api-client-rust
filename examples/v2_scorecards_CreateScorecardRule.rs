// Create a new rule returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_scorecards::ScorecardsAPI;
use datadog_api_client::datadogV2::model::CreateRuleRequest;
use datadog_api_client::datadogV2::model::CreateRuleRequestData;
use datadog_api_client::datadogV2::model::RuleAttributesRequest;
use datadog_api_client::datadogV2::model::RuleType;

#[tokio::main]
async fn main() {
    let body = CreateRuleRequest::new().data(CreateRuleRequestData::new(
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
    let resp = api.create_scorecard_rule(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
