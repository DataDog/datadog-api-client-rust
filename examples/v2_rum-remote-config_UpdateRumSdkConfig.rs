// Update a RUM SDK configuration returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_remote_config::RUMRemoteConfigAPI;
use datadog_api_client::datadogV2::model::RumSdkConfigDynamicOption;
use datadog_api_client::datadogV2::model::RumSdkConfigDynamicOptionPair;
use datadog_api_client::datadogV2::model::RumSdkConfigDynamicOptionSerializedType;
use datadog_api_client::datadogV2::model::RumSdkConfigDynamicOptionStrategy;
use datadog_api_client::datadogV2::model::RumSdkConfigMatchOption;
use datadog_api_client::datadogV2::model::RumSdkConfigMatchOptionSerializedType;
use datadog_api_client::datadogV2::model::RumSdkConfigRumUpdateAttributes;
use datadog_api_client::datadogV2::model::RumSdkConfigSerializedRegex;
use datadog_api_client::datadogV2::model::RumSdkConfigSerializedRegexType;
use datadog_api_client::datadogV2::model::RumSdkConfigTracingUrlConfig;
use datadog_api_client::datadogV2::model::RumSdkConfigTracingUrlPropagatorType;
use datadog_api_client::datadogV2::model::RumSdkConfigType;
use datadog_api_client::datadogV2::model::RumSdkConfigUpdateAttributes;
use datadog_api_client::datadogV2::model::RumSdkConfigUpdateData;
use datadog_api_client::datadogV2::model::RumSdkConfigUpdateRequest;

#[tokio::main]
async fn main() {
    let body = RumSdkConfigUpdateRequest::new(RumSdkConfigUpdateData::new(
        RumSdkConfigUpdateAttributes::new(
            RumSdkConfigRumUpdateAttributes::new("mask".to_string(), true, 20, 75)
                .allowed_tracing_urls(vec![RumSdkConfigTracingUrlConfig::new(
                    RumSdkConfigMatchOption::new(
                        RumSdkConfigMatchOptionSerializedType::STRING,
                        "https://app.datadoghq.com".to_string(),
                    ),
                    vec![
                        RumSdkConfigTracingUrlPropagatorType::DATADOG,
                        RumSdkConfigTracingUrlPropagatorType::TRACECONTEXT,
                    ],
                )])
                .allowed_tracking_origins(vec![RumSdkConfigMatchOption::new(
                    RumSdkConfigMatchOptionSerializedType::STRING,
                    "https://app.datadoghq.com".to_string(),
                )])
                .context(vec![RumSdkConfigDynamicOptionPair::new(
                    "id".to_string(),
                    RumSdkConfigDynamicOption::new(
                        RumSdkConfigDynamicOptionSerializedType::DYNAMIC,
                        RumSdkConfigDynamicOptionStrategy::JS,
                    )
                    .attribute("data-version".to_string())
                    .extractor(RumSdkConfigSerializedRegex::new(
                        RumSdkConfigSerializedRegexType::REGEX,
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
                .user(vec![RumSdkConfigDynamicOptionPair::new(
                    "id".to_string(),
                    RumSdkConfigDynamicOption::new(
                        RumSdkConfigDynamicOptionSerializedType::DYNAMIC,
                        RumSdkConfigDynamicOptionStrategy::JS,
                    )
                    .attribute("data-version".to_string())
                    .extractor(RumSdkConfigSerializedRegex::new(
                        RumSdkConfigSerializedRegexType::REGEX,
                        "^https://app-.*.datadoghq.com".to_string(),
                    ))
                    .key("app.version".to_string())
                    .name("app_version".to_string())
                    .path("application.version".to_string())
                    .selector("#app-version".to_string()),
                )])
                .version(
                    RumSdkConfigDynamicOption::new(
                        RumSdkConfigDynamicOptionSerializedType::DYNAMIC,
                        RumSdkConfigDynamicOptionStrategy::JS,
                    )
                    .attribute("data-version".to_string())
                    .extractor(RumSdkConfigSerializedRegex::new(
                        RumSdkConfigSerializedRegexType::REGEX,
                        "^https://app-.*.datadoghq.com".to_string(),
                    ))
                    .key("app.version".to_string())
                    .name("app_version".to_string())
                    .path("application.version".to_string())
                    .selector("#app-version".to_string()),
                ),
        ),
        "abc12345-1234-5678-abcd-ef1234567890".to_string(),
        RumSdkConfigType::RUM_SDK_CONFIG,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdateRumSdkConfig", true);
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
