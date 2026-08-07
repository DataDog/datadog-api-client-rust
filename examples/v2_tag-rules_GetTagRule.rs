// Get a tag rule returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_tag_rules::GetTagRuleOptionalParams;
use datadog_api_client::datadogV2::api_tag_rules::TagRulesAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.GetTagRule", true);
    let api = TagRulesAPI::with_config(configuration);
    let resp = api
        .get_tag_rule("policy_id".to_string(), GetTagRuleOptionalParams::default())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
