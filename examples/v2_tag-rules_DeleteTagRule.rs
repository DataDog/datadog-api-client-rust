// Delete a tag rule returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_tag_rules::DeleteTagRuleOptionalParams;
use datadog_api_client::datadogV2::api_tag_rules::TagRulesAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.DeleteTagRule", true);
    let api = TagRulesAPI::with_config(configuration);
    let resp = api
        .delete_tag_rule(
            "policy_id".to_string(),
            DeleteTagRuleOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
