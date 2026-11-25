// Update a RUM SDK configuration returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_remote_config::RUMRemoteConfigAPI;
use datadog_api_client::datadogV2::model::RUMSdkConfigDynamicOption;
use datadog_api_client::datadogV2::model::RUMSdkConfigDynamicOptionPair;
use datadog_api_client::datadogV2::model::RUMSdkConfigDynamicOptionSerializedType;
use datadog_api_client::datadogV2::model::RUMSdkConfigDynamicOptionStrategy;
use datadog_api_client::datadogV2::model::RUMSdkConfigMatchOption;
use datadog_api_client::datadogV2::model::RUMSdkConfigMatchOptionSerializedType;
use datadog_api_client::datadogV2::model::RUMSdkConfigRUMUpdateAttributes;
use datadog_api_client::datadogV2::model::RUMSdkConfigSerializedRegex;
use datadog_api_client::datadogV2::model::RUMSdkConfigSerializedRegexType;
use datadog_api_client::datadogV2::model::RUMSdkConfigTracingUrlConfig;
use datadog_api_client::datadogV2::model::RUMSdkConfigTracingUrlPropagatorType;
use datadog_api_client::datadogV2::model::RUMSdkConfigType;
use datadog_api_client::datadogV2::model::RUMSdkConfigUpdateAttributes;
use datadog_api_client::datadogV2::model::RUMSdkConfigUpdateData;
use datadog_api_client::datadogV2::model::RUMSdkConfigUpdateRequest;

#[tokio::main]
async fn main() {
    let body = RUMSdkConfigUpdateRequest::new(RUMSdkConfigUpdateData::new(
        RUMSdkConfigUpdateAttributes::new(
            RUMSdkConfigRUMUpdateAttributes::new("mask".to_string(), true, 20, 75)
                .allowed_tracing_urls(vec![RUMSdkConfigTracingUrlConfig::new(
                    RUMSdkConfigMatchOption::new(
                        RUMSdkConfigMatchOptionSerializedType::STRING,
                        "https://app.datadoghq.com".to_string(),
                    ),
                    vec![
                        RUMSdkConfigTracingUrlPropagatorType::DATADOG,
                        RUMSdkConfigTracingUrlPropagatorType::TRACECONTEXT,
                    ],
                )])
                .allowed_tracking_origins(vec![RUMSdkConfigMatchOption::new(
                    RUMSdkConfigMatchOptionSerializedType::STRING,
                    "https://app.datadoghq.com".to_string(),
                )])
                .context(vec![RUMSdkConfigDynamicOptionPair::new(
                    "id".to_string(),
                    RUMSdkConfigDynamicOption::new(
                        RUMSdkConfigDynamicOptionSerializedType::DYNAMIC,
                        RUMSdkConfigDynamicOptionStrategy::JS,
                    )
                    .attribute("data-version".to_string())
                    .extractor(RUMSdkConfigSerializedRegex::new(
                        RUMSdkConfigSerializedRegexType::REGEX,
                        "^https://app-.*.datadoghq.com".to_string(),
                    ))
                    .key("app.version".to_string())
                    .name("app_version".to_string())
                    .path("application.version".to_string())
                    .selector("#app-version".to_string()),
                )])
                .env("production".to_string())
                .service("my-service".to_string())
                .trace_sample_rate(100)
                .track_session_across_subdomains(false)
                .user(vec![RUMSdkConfigDynamicOptionPair::new(
                    "id".to_string(),
                    RUMSdkConfigDynamicOption::new(
                        RUMSdkConfigDynamicOptionSerializedType::DYNAMIC,
                        RUMSdkConfigDynamicOptionStrategy::JS,
                    )
                    .attribute("data-version".to_string())
                    .extractor(RUMSdkConfigSerializedRegex::new(
                        RUMSdkConfigSerializedRegexType::REGEX,
                        "^https://app-.*.datadoghq.com".to_string(),
                    ))
                    .key("app.version".to_string())
                    .name("app_version".to_string())
                    .path("application.version".to_string())
                    .selector("#app-version".to_string()),
                )])
                .version(
                    RUMSdkConfigDynamicOption::new(
                        RUMSdkConfigDynamicOptionSerializedType::DYNAMIC,
                        RUMSdkConfigDynamicOptionStrategy::JS,
                    )
                    .attribute("data-version".to_string())
                    .extractor(RUMSdkConfigSerializedRegex::new(
                        RUMSdkConfigSerializedRegexType::REGEX,
                        "^https://app-.*.datadoghq.com".to_string(),
                    ))
                    .key("app.version".to_string())
                    .name("app_version".to_string())
                    .path("application.version".to_string())
                    .selector("#app-version".to_string()),
                ),
        ),
        "abc12345-1234-5678-abcd-ef1234567890".to_string(),
        RUMSdkConfigType::RUM_SDK_CONFIG,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.update_rum_sdk_config", true);
    let api = RUMRemoteConfigAPI::with_config(configuration);
    let resp = api
        .update_rum_sdk_config("config_id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
