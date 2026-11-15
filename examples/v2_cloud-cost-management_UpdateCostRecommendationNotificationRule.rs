// Update a cost recommendation notification rule returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_cloud_cost_management::CloudCostManagementAPI;
use datadog_api_client::datadogV2::model::CostRecommendationNotificationRuleDestinationType;
use datadog_api_client::datadogV2::model::CostRecommendationNotificationRuleFrequency;
use datadog_api_client::datadogV2::model::CostRecommendationNotificationRuleMessageAction;
use datadog_api_client::datadogV2::model::CostRecommendationNotificationRuleMessageActions;
use datadog_api_client::datadogV2::model::CostRecommendationNotificationRuleMessageCustomization;
use datadog_api_client::datadogV2::model::CostRecommendationNotificationRuleMessageCustomizationHeader;
use datadog_api_client::datadogV2::model::CostRecommendationNotificationRuleMessageCustomizationMode;
use datadog_api_client::datadogV2::model::CostRecommendationNotificationRuleMessageCustomizationPerRec;
use datadog_api_client::datadogV2::model::CostRecommendationNotificationRuleType;
use datadog_api_client::datadogV2::model::CostRecommendationNotificationRuleUpdateAttributes;
use datadog_api_client::datadogV2::model::CostRecommendationNotificationRuleUpdateData;
use datadog_api_client::datadogV2::model::CostRecommendationNotificationRuleUpdateRequest;

#[tokio::main]
async fn main() {
    let body = CostRecommendationNotificationRuleUpdateRequest::new(
        CostRecommendationNotificationRuleUpdateData::new(
            CostRecommendationNotificationRuleUpdateAttributes::new()
                .channel_id("C0456ABCD".to_string())
                .destination_type(CostRecommendationNotificationRuleDestinationType::SLACK)
                .enabled(false)
                .frequency(CostRecommendationNotificationRuleFrequency::WEEKLY)
                .interval_execution_date("2026-11-02".to_string())
                .interval_execution_time("09:30".to_string())
                .mention_targets(vec!["U01ABCDE".to_string()])
                .message_customization(
                    CostRecommendationNotificationRuleMessageCustomization::new()
                        .actions(
                            CostRecommendationNotificationRuleMessageActions::new().view_in_ccm(
                                CostRecommendationNotificationRuleMessageAction::new()
                                    .bulk(true)
                                    .per_rec(true),
                            ),
                        )
                        .header(
                            CostRecommendationNotificationRuleMessageCustomizationHeader::new()
                                .custom_text(" recommendations to review this week.".to_string())
                                .notification_name(true)
                                .total_savings(true),
                        )
                        .include_tags(vec!["team".to_string()])
                        .mode(
                            CostRecommendationNotificationRuleMessageCustomizationMode::STRUCTURED,
                        )
                        .per_rec(
                            CostRecommendationNotificationRuleMessageCustomizationPerRec::new()
                                .effort(true)
                                .risk(true)
                                .why(true),
                        ),
                )
                .name("Production recommendations".to_string())
                .scope_query("service:api AND env:prod".to_string())
                .timezone("America/New_York".to_string())
                .weekly_execution_day("monday".to_string())
                .weekly_execution_time("09:30".to_string())
                .workspace_id("T0123ABCD".to_string()),
            "2c59b248-6a91-4f95-972a-dc236cae9cbb".to_string(),
            CostRecommendationNotificationRuleType::NOTIFICATION_RULES,
        ),
    );
    let mut configuration = datadog::Configuration::new();
    configuration
        .set_unstable_operation_enabled("v2.update_cost_recommendation_notification_rule", true);
    let api = CloudCostManagementAPI::with_config(configuration);
    let resp = api
        .update_cost_recommendation_notification_rule("notification_rule_id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
