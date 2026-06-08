// Update a tag indexing rule returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_metrics::MetricsAPI;
use datadog_api_client::datadogV2::model::TagIndexingRuleDynamicTags;
use datadog_api_client::datadogV2::model::TagIndexingRuleMetricMatch;
use datadog_api_client::datadogV2::model::TagIndexingRuleOptions;
use datadog_api_client::datadogV2::model::TagIndexingRuleOptionsData;
use datadog_api_client::datadogV2::model::TagIndexingRuleType;
use datadog_api_client::datadogV2::model::TagIndexingRuleUpdateAttributes;
use datadog_api_client::datadogV2::model::TagIndexingRuleUpdateData;
use datadog_api_client::datadogV2::model::TagIndexingRuleUpdateRequest;

#[tokio::main]
async fn main() {
    // there is a valid "tag_indexing_rule" in the system
    let tag_indexing_rule_data_id = std::env::var("TAG_INDEXING_RULE_DATA_ID").unwrap();
    let body = TagIndexingRuleUpdateRequest::new(
        TagIndexingRuleUpdateData::new(TagIndexingRuleType::TAG_INDEXING_RULES).attributes(
            TagIndexingRuleUpdateAttributes::new()
                .ignored_metric_name_matches(vec![])
                .metric_name_matches(vec!["dd.test.*".to_string()])
                .name("my-indexing-rule".to_string())
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
                .rule_order(2)
                .tags(vec!["env".to_string(), "service".to_string()]),
        ),
    );
    let configuration = datadog::Configuration::new();
    let api = MetricsAPI::with_config(configuration);
    let resp = api
        .update_tag_indexing_rule(tag_indexing_rule_data_id.clone(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
