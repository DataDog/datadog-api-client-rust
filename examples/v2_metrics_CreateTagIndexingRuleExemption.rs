// Create a tag indexing rule exemption returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_metrics::MetricsAPI;
use datadog_api_client::datadogV2::model::TagIndexingRuleExemptionCreateAttributes;
use datadog_api_client::datadogV2::model::TagIndexingRuleExemptionCreateData;
use datadog_api_client::datadogV2::model::TagIndexingRuleExemptionCreateRequest;
use datadog_api_client::datadogV2::model::TagIndexingRuleExemptionType;

#[tokio::main]
async fn main() {
    let body = TagIndexingRuleExemptionCreateRequest::new(TagIndexingRuleExemptionCreateData::new(
        TagIndexingRuleExemptionCreateAttributes::new(
            "This metric has a pre-existing tag configuration.".to_string(),
        ),
        TagIndexingRuleExemptionType::TAG_INDEXING_RULE_EXEMPTIONS,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateTagIndexingRuleExemption", true);
    let api = MetricsAPI::with_config(configuration);
    let resp = api
        .create_tag_indexing_rule_exemption("metric_name".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
