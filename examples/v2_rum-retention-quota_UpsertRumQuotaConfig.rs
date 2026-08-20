// Create or update a RUM retention quota config returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_retention_quota::RUMRetentionQuotaAPI;
use datadog_api_client::datadogV2::model::RumRetentionQuotaAdaptiveConfig;
use datadog_api_client::datadogV2::model::RumRetentionQuotaConfigType;
use datadog_api_client::datadogV2::model::RumRetentionQuotaConfigUpdateAttributes;
use datadog_api_client::datadogV2::model::RumRetentionQuotaConfigUpdateData;
use datadog_api_client::datadogV2::model::RumRetentionQuotaConfigUpdateRequest;
use datadog_api_client::datadogV2::model::RumRetentionQuotaCustomConfig;
use datadog_api_client::datadogV2::model::RumRetentionQuotaMode;
use datadog_api_client::datadogV2::model::RumRetentionQuotaReachedAction;
use datadog_api_client::datadogV2::model::RumRetentionQuotaScopeType;
use datadog_api_client::datadogV2::model::RumRetentionQuotaWindowType;

#[tokio::main]
async fn main() {
    let body = RumRetentionQuotaConfigUpdateRequest::new(RumRetentionQuotaConfigUpdateData::new(
        RumRetentionQuotaConfigUpdateAttributes::new(RumRetentionQuotaMode::CUSTOM)
            .adaptive(RumRetentionQuotaAdaptiveConfig::new(0.5))
            .custom(RumRetentionQuotaCustomConfig::new(
                "08:00".to_string(),
                "+09:00".to_string(),
                RumRetentionQuotaReachedAction::STOP,
                1000000,
                RumRetentionQuotaWindowType::DAILY,
            )),
        "ced16651-97b6-4e67-8590-8caec3af0695".to_string(),
        RumRetentionQuotaConfigType::RUM_QUOTA_CONFIG,
    ));
    let configuration = datadog::Configuration::new();
    let api = RUMRetentionQuotaAPI::with_config(configuration);
    let resp = api
        .upsert_rum_quota_config(
            RumRetentionQuotaScopeType::APPLICATION,
            "ced16651-97b6-4e67-8590-8caec3af0695".to_string(),
            body,
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
