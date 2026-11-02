// Reorder tag indexing rules returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_metrics::MetricsAPI;
use datadog_api_client::datadogV2::model::TagIndexingRuleOrderAttributes;
use datadog_api_client::datadogV2::model::TagIndexingRuleOrderData;
use datadog_api_client::datadogV2::model::TagIndexingRuleOrderRequest;
use datadog_api_client::datadogV2::model::TagIndexingRuleType;

#[tokio::main]
async fn main() {
    // there is a valid "tag_indexing_rule" in the system
    let tag_indexing_rule_data_id = std::env::var("TAG_INDEXING_RULE_DATA_ID").unwrap();
    let body = TagIndexingRuleOrderRequest::new(TagIndexingRuleOrderData::new(
        TagIndexingRuleOrderAttributes::new().rule_ids(vec![tag_indexing_rule_data_id.clone()]),
        TagIndexingRuleType::TAG_INDEXING_RULES,
    ));
    let configuration = datadog::Configuration::new();
    let api = MetricsAPI::with_config(configuration);
    let resp = api.reorder_tag_indexing_rules(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
