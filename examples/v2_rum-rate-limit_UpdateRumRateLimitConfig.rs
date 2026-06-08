// Create or update a RUM rate limit configuration returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_rate_limit::RumRateLimitAPI;
use datadog_api_client::datadogV2::model::RumRateLimitAdaptiveConfig;
use datadog_api_client::datadogV2::model::RumRateLimitConfigType;
use datadog_api_client::datadogV2::model::RumRateLimitConfigUpdateAttributes;
use datadog_api_client::datadogV2::model::RumRateLimitConfigUpdateData;
use datadog_api_client::datadogV2::model::RumRateLimitConfigUpdateRequest;
use datadog_api_client::datadogV2::model::RumRateLimitCustomConfig;
use datadog_api_client::datadogV2::model::RumRateLimitMode;
use datadog_api_client::datadogV2::model::RumRateLimitQuotaReachedAction;
use datadog_api_client::datadogV2::model::RumRateLimitScopeType;
use datadog_api_client::datadogV2::model::RumRateLimitWindowType;

#[tokio::main]
async fn main() {
    let body = RumRateLimitConfigUpdateRequest::new(RumRateLimitConfigUpdateData::new(
        RumRateLimitConfigUpdateAttributes::new(RumRateLimitMode::CUSTOM)
            .adaptive(RumRateLimitAdaptiveConfig::new(0.5))
            .custom(RumRateLimitCustomConfig::new(
                "08:00".to_string(),
                "+09:00".to_string(),
                RumRateLimitQuotaReachedAction::STOP,
                1000000,
                RumRateLimitWindowType::DAILY,
            )),
        "cd73a516-a481-4af5-8352-9b577465c77b".to_string(),
        RumRateLimitConfigType::RUM_RATE_LIMIT_CONFIG,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdateRumRateLimitConfig", true);
    let api = RumRateLimitAPI::with_config(configuration);
    let resp = api
        .update_rum_rate_limit_config(
            RumRateLimitScopeType::APPLICATION,
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
