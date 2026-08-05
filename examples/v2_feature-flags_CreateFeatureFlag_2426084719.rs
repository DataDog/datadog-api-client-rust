// Create a feature flag with notification rule targets returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_feature_flags::FeatureFlagsAPI;
use datadog_api_client::datadogV2::model::CreateFeatureFlagAttributes;
use datadog_api_client::datadogV2::model::CreateFeatureFlagData;
use datadog_api_client::datadogV2::model::CreateFeatureFlagDataType;
use datadog_api_client::datadogV2::model::CreateFeatureFlagRequest;
use datadog_api_client::datadogV2::model::CreateVariant;
use datadog_api_client::datadogV2::model::NotificationRuleTarget;
use datadog_api_client::datadogV2::model::NotificationRuleTargetConfiguration;
use datadog_api_client::datadogV2::model::NotificationRuleTargetType;
use datadog_api_client::datadogV2::model::ValueType;

#[tokio::main]
async fn main() {
    let body = CreateFeatureFlagRequest::new(CreateFeatureFlagData::new(
        CreateFeatureFlagAttributes::new(
            "Test feature flag with notification rule targets for BDD scenarios".to_string(),
            "test-feature-flag-notify-Example-Feature-Flag".to_string(),
            "Test Feature Flag Notify Example-Feature-Flag".to_string(),
            ValueType::BOOLEAN,
            vec![
                CreateVariant::new(
                    "variant-Example-Feature-Flag-1".to_string(),
                    "Variant Example-Feature-Flag A".to_string(),
                    "true".to_string(),
                ),
                CreateVariant::new(
                    "variant-Example-Feature-Flag-2".to_string(),
                    "Variant Example-Feature-Flag B".to_string(),
                    "false".to_string(),
                ),
            ],
        )
        .default_variant_key(Some("variant-Example-Feature-Flag-1".to_string()))
        .notification_rule_query(Some("notification_type:rollout_started".to_string()))
        .rule_targets(vec![NotificationRuleTarget::new(
            NotificationRuleTargetConfiguration::new()
                .channel("#feature-flags-test".to_string())
                .workspace("datadoghq".to_string()),
            NotificationRuleTargetType::SLACK_CHANNEL,
            1,
        )]),
        CreateFeatureFlagDataType::FEATURE_FLAGS,
    ));
    let configuration = datadog::Configuration::new();
    let api = FeatureFlagsAPI::with_config(configuration);
    let resp = api.create_feature_flag(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
