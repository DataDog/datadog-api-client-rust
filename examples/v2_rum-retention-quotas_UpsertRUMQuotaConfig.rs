// Create or update a RUM retention quota config returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_retention_quotas::RUMRetentionQuotasAPI;
use datadog_api_client::datadogV2::model::RUMRetentionQuotaConfigType;
use datadog_api_client::datadogV2::model::RUMRetentionQuotaConfigUpdateAttributes;
use datadog_api_client::datadogV2::model::RUMRetentionQuotaConfigUpdateData;
use datadog_api_client::datadogV2::model::RUMRetentionQuotaConfigUpdateRequest;
use datadog_api_client::datadogV2::model::RUMRetentionQuotaCustomConfig;
use datadog_api_client::datadogV2::model::RUMRetentionQuotaMode;
use datadog_api_client::datadogV2::model::RUMRetentionQuotaReachedAction;
use datadog_api_client::datadogV2::model::RUMRetentionQuotaScopeType;
use datadog_api_client::datadogV2::model::RUMRetentionQuotaWindowType;

#[tokio::main]
async fn main() {
    let body = RUMRetentionQuotaConfigUpdateRequest::new(RUMRetentionQuotaConfigUpdateData::new(
        RUMRetentionQuotaConfigUpdateAttributes::new(RUMRetentionQuotaMode::CUSTOM).custom(
            RUMRetentionQuotaCustomConfig::new(
                "08:00".to_string(),
                "+09:00".to_string(),
                RUMRetentionQuotaReachedAction::STOP,
                1000000,
                RUMRetentionQuotaWindowType::DAILY,
            ),
        ),
        "cd73a516-a481-4af5-8352-9b577465c77b".to_string(),
        RUMRetentionQuotaConfigType::RUM_QUOTA_CONFIG,
    ));
    let configuration = datadog::Configuration::new();
    let api = RUMRetentionQuotasAPI::with_config(configuration);
    let resp = api
        .upsert_rum_quota_config(
            RUMRetentionQuotaScopeType::APPLICATION,
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
