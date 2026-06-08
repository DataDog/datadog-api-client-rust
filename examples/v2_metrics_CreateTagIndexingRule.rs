// Create a tag indexing rule returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_metrics::MetricsAPI;
use datadog_api_client::datadogV2::model::TagIndexingRuleCreateAttributes;
use datadog_api_client::datadogV2::model::TagIndexingRuleCreateData;
use datadog_api_client::datadogV2::model::TagIndexingRuleCreateRequest;
use datadog_api_client::datadogV2::model::TagIndexingRuleDynamicTags;
use datadog_api_client::datadogV2::model::TagIndexingRuleMetricMatch;
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
        .exclude_tags_mode(false)
        .ignored_metric_name_matches(vec![])
        .options(
            TagIndexingRuleOptions::new()
                .data(
                    TagIndexingRuleOptionsData::new()
                        .dynamic_tags(
                            TagIndexingRuleDynamicTags::new()
                                .queried_tags_window_seconds(3600)
                                .related_asset_tags(false),
                        )
                        .manage_preexisting_metrics(true)
                        .metric_match(
                            TagIndexingRuleMetricMatch::new().queried_window_seconds(3600),
                        )
                        .override_previous_rules(false),
                )
                .version(1),
        )
        .tags(vec!["env".to_string(), "service".to_string()]),
        TagIndexingRuleType::TAG_INDEXING_RULES,
    ));
    let configuration = datadog::Configuration::new();
    let api = MetricsAPI::with_config(configuration);
    let resp = api.create_tag_indexing_rule(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
