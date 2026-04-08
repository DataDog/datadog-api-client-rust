// Update an existing rule returns "Rule updated successfully" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_scorecards::ScorecardsAPI;
use datadog_api_client::datadogV2::model::RuleAttributesRequest;
use datadog_api_client::datadogV2::model::RuleType;
use datadog_api_client::datadogV2::model::UpdateRuleRequest;
use datadog_api_client::datadogV2::model::UpdateRuleRequestData;

#[tokio::main]
async fn main() {
    // there is a valid "create_scorecard_rule" in the system
    let create_scorecard_rule_data_attributes_name =
        std::env::var("CREATE_SCORECARD_RULE_DATA_ATTRIBUTES_NAME").unwrap();
    let create_scorecard_rule_data_attributes_scorecard_name =
        std::env::var("CREATE_SCORECARD_RULE_DATA_ATTRIBUTES_SCORECARD_NAME").unwrap();
    let create_scorecard_rule_data_id = std::env::var("CREATE_SCORECARD_RULE_DATA_ID").unwrap();
    let body = UpdateRuleRequest::new().data(
        UpdateRuleRequestData::new()
            .attributes(
                RuleAttributesRequest::new()
                    .description("Updated description via test".to_string())
                    .enabled(true)
                    .name(create_scorecard_rule_data_attributes_name.clone())
                    .scorecard_name(create_scorecard_rule_data_attributes_scorecard_name.clone()),
            )
            .type_(RuleType::RULE),
    );
    let configuration = datadog::Configuration::new();
    let api = ScorecardsAPI::with_config(configuration);
    let resp = api
        .update_scorecard_rule(create_scorecard_rule_data_id.clone(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
