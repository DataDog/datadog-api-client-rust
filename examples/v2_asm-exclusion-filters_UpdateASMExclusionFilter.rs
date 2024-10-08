// Update a ASM Exclusion filter returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_asm_exclusion_filters::ASMExclusionFiltersAPI;
use datadog_api_client::datadogV2::model::ASMExclusionFilterRulesTarget;
use datadog_api_client::datadogV2::model::ASMExclusionFilterScope;
use datadog_api_client::datadogV2::model::ASMExclusionFilterType;
use datadog_api_client::datadogV2::model::ASMExclusionFilterUpdateAttributes;
use datadog_api_client::datadogV2::model::ASMExclusionFilterUpdateData;
use datadog_api_client::datadogV2::model::ASMExclusionFilterUpdateRequest;

#[tokio::main]
async fn main() {
    let body = ASMExclusionFilterUpdateRequest::new(
        ASMExclusionFilterUpdateData::new(
            ASMExclusionFilterUpdateAttributes::new(
                "My Exclusion filter".to_string(),
                true,
                "/lfi_include/*".to_string(),
            )
            .ip_list(vec!["127.0.0.1".to_string()])
            .parameters(vec!["list.search.query".to_string()])
            .rules_target(vec![
                ASMExclusionFilterRulesTarget::new().rule_id("dog-913-009".to_string())
            ])
            .scope(vec![ASMExclusionFilterScope::new()
                .env("dd-appsec-php-support".to_string())
                .service("anil-php-weblog".to_string())]),
            ASMExclusionFilterType::EXCLUSION_FILTER,
        )
        .id("3dd-0uc-h1s".to_string()),
    );
    let configuration = datadog::Configuration::new();
    let api = ASMExclusionFiltersAPI::with_config(configuration);
    let resp = api
        .update_asm_exclusion_filter("exclusion_filter_id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
