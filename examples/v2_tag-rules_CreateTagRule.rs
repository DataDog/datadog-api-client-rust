// Create a tag rule returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_tag_rules::TagRulesAPI;
use datadog_api_client::datadogV2::model::TagRuleCreateAttributes;
use datadog_api_client::datadogV2::model::TagRuleCreateData;
use datadog_api_client::datadogV2::model::TagRuleCreateRequest;
use datadog_api_client::datadogV2::model::TagRuleCreateType;
use datadog_api_client::datadogV2::model::TagRuleResourceType;
use datadog_api_client::datadogV2::model::TagRuleSource;

#[tokio::main]
async fn main() {
    let body = TagRuleCreateRequest::new(TagRuleCreateData::new(
        TagRuleCreateAttributes::new(
            "Service tag must be one of api or web".to_string(),
            TagRuleCreateType::SURFACING,
            "env".to_string(),
            TagRuleSource::LOGS,
            "service".to_string(),
            vec!["api".to_string(), "web".to_string()],
        )
        .enabled(true)
        .negated(false)
        .required(true),
        TagRuleResourceType::TAG_RULE,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateTagRule", true);
    let api = TagRulesAPI::with_config(configuration);
    let resp = api.create_tag_rule(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
