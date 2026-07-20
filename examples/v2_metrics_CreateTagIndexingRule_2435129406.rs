// Create a tag indexing rule with exclude-mode tag usage fields returns "Created"
// response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_metrics::MetricsAPI;
use datadog_api_client::datadogV2::model::TagIndexingRuleCreateAttributes;
use datadog_api_client::datadogV2::model::TagIndexingRuleCreateData;
use datadog_api_client::datadogV2::model::TagIndexingRuleCreateRequest;
use datadog_api_client::datadogV2::model::TagIndexingRuleDynamicTags;
use datadog_api_client::datadogV2::model::TagIndexingRuleOptions;
use datadog_api_client::datadogV2::model::TagIndexingRuleOptionsData;
use datadog_api_client::datadogV2::model::TagIndexingRuleType;

#[tokio::main]
async fn main() {
    let body = TagIndexingRuleCreateRequest::new(TagIndexingRuleCreateData::new(
        TagIndexingRuleCreateAttributes::new(
            vec!["dd.test.*".to_string()],
            "my-indexing-rule".to_string(),
        )
        .exclude_tags_mode(true)
        .ignored_metric_name_matches(vec![])
        .options(
            TagIndexingRuleOptions::new()
                .data(
                    TagIndexingRuleOptionsData::new()
                        .dynamic_tags(
                            TagIndexingRuleDynamicTags::new()
                                .exclude_not_queried_window_seconds(3600)
                                .exclude_not_used_in_assets(true),
                        )
                        .manage_preexisting_metrics(true)
                        .override_previous_rules(false),
                )
                .version(1),
        )
        .tags(vec!["env".to_string(), "service".to_string()]),
        TagIndexingRuleType::TAG_INDEXING_RULES,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateTagIndexingRule", true);
    let api = MetricsAPI::with_config(configuration);
    let resp = api.create_tag_indexing_rule(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
