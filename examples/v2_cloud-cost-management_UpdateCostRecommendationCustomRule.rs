// Update a cost recommendation custom rule returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_cloud_cost_management::CloudCostManagementAPI;
use datadog_api_client::datadogV2::model::CostRecommendationCustomRuleFilter;
use datadog_api_client::datadogV2::model::CostRecommendationCustomRuleFilterType;
use datadog_api_client::datadogV2::model::CostRecommendationCustomRuleType;
use datadog_api_client::datadogV2::model::CostRecommendationCustomRuleUpdateAttributes;
use datadog_api_client::datadogV2::model::CostRecommendationCustomRuleUpdateData;
use datadog_api_client::datadogV2::model::CostRecommendationCustomRuleUpdateRequest;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let body = CostRecommendationCustomRuleUpdateRequest::new(
        CostRecommendationCustomRuleUpdateData::new(
            CostRecommendationCustomRuleUpdateAttributes::new()
                .enabled(false)
                .filters(vec![CostRecommendationCustomRuleFilter::new(
                    CostRecommendationCustomRuleFilterType::IS,
                    "env".to_string(),
                )
                .values(vec!["staging".to_string()])])
                .name("Staging and development instances".to_string())
                .notes("Staging instances can run with less headroom.".to_string())
                .settings(BTreeMap::from([
                    ("lookback_days".to_string(), 14.0),
                    ("max_cpu_user_limit".to_string(), 0.05),
                ])),
            "8a1b6f3e-2c4d-4e5f-9a7b-1c2d3e4f5a6b".to_string(),
            CostRecommendationCustomRuleType::CUSTOM_RULES,
        ),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.update_cost_recommendation_custom_rule", true);
    let api = CloudCostManagementAPI::with_config(configuration);
    let resp = api
        .update_cost_recommendation_custom_rule("custom_rule_id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
