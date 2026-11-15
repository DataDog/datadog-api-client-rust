// Create a cost recommendation custom rule returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_cloud_cost_management::CloudCostManagementAPI;
use datadog_api_client::datadogV2::model::CostRecommendationCustomRuleCreateAttributes;
use datadog_api_client::datadogV2::model::CostRecommendationCustomRuleCreateData;
use datadog_api_client::datadogV2::model::CostRecommendationCustomRuleCreateRequest;
use datadog_api_client::datadogV2::model::CostRecommendationCustomRuleFilter;
use datadog_api_client::datadogV2::model::CostRecommendationCustomRuleFilterType;
use datadog_api_client::datadogV2::model::CostRecommendationCustomRuleType;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let body = CostRecommendationCustomRuleCreateRequest::new(
        CostRecommendationCustomRuleCreateData::new(
            CostRecommendationCustomRuleCreateAttributes::new(
                "Staging instances".to_string(),
                "aws_ec2_instance_unused".to_string(),
            )
            .enabled(true)
            .filters(vec![CostRecommendationCustomRuleFilter::new(
                CostRecommendationCustomRuleFilterType::IS,
                "env".to_string(),
            )
            .values(vec!["staging".to_string()])])
            .notes("Staging instances can run with less headroom.".to_string())
            .settings(BTreeMap::from([
                ("lookback_days".to_string(), 14.0),
                ("max_cpu_user_limit".to_string(), 0.05),
            ])),
            CostRecommendationCustomRuleType::CUSTOM_RULES,
        ),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.create_cost_recommendation_custom_rule", true);
    let api = CloudCostManagementAPI::with_config(configuration);
    let resp = api.create_cost_recommendation_custom_rule(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
