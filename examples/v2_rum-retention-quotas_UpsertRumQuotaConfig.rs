// Create or update a RUM retention quota config returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_retention_quotas::RUMRetentionQuotasAPI;
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
        RumRetentionQuotaConfigUpdateAttributes::new(RumRetentionQuotaMode::CUSTOM).custom(
            RumRetentionQuotaCustomConfig::new(
                "08:00".to_string(),
                "+09:00".to_string(),
                RumRetentionQuotaReachedAction::STOP,
                1000000,
                RumRetentionQuotaWindowType::DAILY,
            ),
        ),
        "cd73a516-a481-4af5-8352-9b577465c77b".to_string(),
        RumRetentionQuotaConfigType::RUM_QUOTA_CONFIG,
    ));
    let configuration = datadog::Configuration::new();
    let api = RUMRetentionQuotasAPI::with_config(configuration);
    let resp = api
        .upsert_rum_quota_config(
            RumRetentionQuotaScopeType::APPLICATION,
            "cd73a516-a481-4af5-8352-9b577465c77b".to_string(),
            body,
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
