// Create a WAF exclusion filter returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_application_security::ApplicationSecurityAPI;
use datadog_api_client::datadogV2::model::ApplicationSecurityWafExclusionFilterCreateAttributes;
use datadog_api_client::datadogV2::model::ApplicationSecurityWafExclusionFilterCreateData;
use datadog_api_client::datadogV2::model::ApplicationSecurityWafExclusionFilterCreateRequest;
use datadog_api_client::datadogV2::model::ApplicationSecurityWafExclusionFilterRulesTarget;
use datadog_api_client::datadogV2::model::ApplicationSecurityWafExclusionFilterRulesTargetTags;
use datadog_api_client::datadogV2::model::ApplicationSecurityWafExclusionFilterScope;
use datadog_api_client::datadogV2::model::ApplicationSecurityWafExclusionFilterType;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let body = ApplicationSecurityWafExclusionFilterCreateRequest::new(
        ApplicationSecurityWafExclusionFilterCreateData::new(
            ApplicationSecurityWafExclusionFilterCreateAttributes::new(
                "Exclude false positives on a path".to_string(),
                true,
            )
            .parameters(vec!["list.search.query".to_string()])
            .path_glob("/accounts/*".to_string())
            .rules_target(vec![ApplicationSecurityWafExclusionFilterRulesTarget::new(
            )
            .tags(
                ApplicationSecurityWafExclusionFilterRulesTargetTags::new()
                    .category("attack_attempt".to_string())
                    .type_("lfi".to_string())
                    .additional_properties(BTreeMap::from([])),
            )])
            .scope(vec![ApplicationSecurityWafExclusionFilterScope::new()
                .env("www".to_string())
                .service("prod".to_string())]),
            ApplicationSecurityWafExclusionFilterType::EXCLUSION_FILTER,
        ),
    );
    let configuration = datadog::Configuration::new();
    let api = ApplicationSecurityAPI::with_config(configuration);
    let resp = api
        .create_application_security_waf_exclusion_filter(body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
