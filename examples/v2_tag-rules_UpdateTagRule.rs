// Update a tag rule returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_tag_rules::TagRulesAPI;
use datadog_api_client::datadogV2::model::TagRuleResourceType;
use datadog_api_client::datadogV2::model::TagRuleType;
use datadog_api_client::datadogV2::model::TagRuleUpdateAttributes;
use datadog_api_client::datadogV2::model::TagRuleUpdateData;
use datadog_api_client::datadogV2::model::TagRuleUpdateRequest;

#[tokio::main]
async fn main() {
    let body = TagRuleUpdateRequest::new(
        TagRuleUpdateData::new("123".to_string(), TagRuleResourceType::TAG_RULE).attributes(
            TagRuleUpdateAttributes::new()
                .rule_type(TagRuleType::SURFACING)
                .tag_value_patterns(vec![]),
        ),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdateTagRule", true);
    let api = TagRulesAPI::with_config(configuration);
    let resp = api.update_tag_rule("rule_id".to_string(), body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
