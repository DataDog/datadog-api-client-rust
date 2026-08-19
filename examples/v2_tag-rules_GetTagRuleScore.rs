// Get a tag rule compliance score returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_tag_rules::GetTagRuleScoreOptionalParams;
use datadog_api_client::datadogV2::api_tag_rules::TagRulesAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.GetTagRuleScore", true);
    let api = TagRulesAPI::with_config(configuration);
    let resp = api
        .get_tag_rule_score(
            "rule_id".to_string(),
            GetTagRuleScoreOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
