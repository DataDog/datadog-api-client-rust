// Update an ASM Exclusion filter returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_application_security_exclusion_filters::ApplicationSecurityExclusionFiltersAPI;
use datadog_api_client::datadogV2::model::ApplicationSecurityExclusionFilterRulesTarget;
use datadog_api_client::datadogV2::model::ApplicationSecurityExclusionFilterScope;
use datadog_api_client::datadogV2::model::ApplicationSecurityExclusionFilterType;
use datadog_api_client::datadogV2::model::ApplicationSecurityExclusionFilterUpdateAttributes;
use datadog_api_client::datadogV2::model::ApplicationSecurityExclusionFilterUpdateData;
use datadog_api_client::datadogV2::model::ApplicationSecurityExclusionFilterUpdateRequest;

#[tokio::main]
async fn main() {
    let body = ApplicationSecurityExclusionFilterUpdateRequest::new(
        ApplicationSecurityExclusionFilterUpdateData::new(
            ApplicationSecurityExclusionFilterUpdateAttributes::new(
                "My Exclusion filter".to_string(),
                true,
                "/lfi_include/*".to_string(),
            )
            .ip_list(vec!["127.0.0.1".to_string()])
            .parameters(vec!["list.search.query".to_string()])
            .rules_target(vec![ApplicationSecurityExclusionFilterRulesTarget::new()
                .rule_id("dog-913-009".to_string())])
            .scope(vec![ApplicationSecurityExclusionFilterScope::new()
                .env("dd-appsec-php-support".to_string())
                .service("anil-php-weblog".to_string())]),
            ApplicationSecurityExclusionFilterType::EXCLUSION_FILTER,
        )
        .id("3dd-0uc-h1s".to_string()),
    );
    let configuration = datadog::Configuration::new();
    let api = ApplicationSecurityExclusionFiltersAPI::with_config(configuration);
    let resp = api
        .update_application_security_exclusion_filter("exclusion_filter_id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
