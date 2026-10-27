// Update a tag indexing rule exemption returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_metrics::MetricsAPI;
use datadog_api_client::datadogV2::model::TagIndexingRuleExemptionType;
use datadog_api_client::datadogV2::model::TagIndexingRuleExemptionUpdateAttributes;
use datadog_api_client::datadogV2::model::TagIndexingRuleExemptionUpdateData;
use datadog_api_client::datadogV2::model::TagIndexingRuleExemptionUpdateRequest;

#[tokio::main]
async fn main() {
    let body = TagIndexingRuleExemptionUpdateRequest::new(TagIndexingRuleExemptionUpdateData::new(
        TagIndexingRuleExemptionUpdateAttributes::new(
            "This metric must remain exempt because of its query requirements.".to_string(),
        ),
        TagIndexingRuleExemptionType::TAG_INDEXING_RULE_EXEMPTIONS,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.update_tag_indexing_rule_exemption", true);
    let api = MetricsAPI::with_config(configuration);
    let resp = api
        .update_tag_indexing_rule_exemption("metric_name".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
