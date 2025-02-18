// Create an Application Security exclusion filter returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_application_security::ApplicationSecurityAPI;
use datadog_api_client::datadogV2::model::ApplicationSecurityExclusionFilterAttributes;
use datadog_api_client::datadogV2::model::ApplicationSecurityExclusionFilterOnMatch;
use datadog_api_client::datadogV2::model::ApplicationSecurityExclusionFilterRequest;
use datadog_api_client::datadogV2::model::ApplicationSecurityExclusionFilterResource;
use datadog_api_client::datadogV2::model::ApplicationSecurityExclusionFilterRulesTarget;
use datadog_api_client::datadogV2::model::ApplicationSecurityExclusionFilterRulesTargetTags;
use datadog_api_client::datadogV2::model::ApplicationSecurityExclusionFilterScope;
use datadog_api_client::datadogV2::model::ApplicationSecurityExclusionFilterType;

#[tokio::main]
async fn main() {
    let body = ApplicationSecurityExclusionFilterRequest::new(
        ApplicationSecurityExclusionFilterResource::new(
            ApplicationSecurityExclusionFilterAttributes::new(
                "Exclude false positives on a path".to_string(),
                true,
            )
            .ip_list(vec!["198.51.100.72".to_string()])
            .on_match(ApplicationSecurityExclusionFilterOnMatch::MONITOR)
            .parameters(vec!["list.search.query".to_string()])
            .path_glob("/accounts/*".to_string())
            .rules_target(vec![ApplicationSecurityExclusionFilterRulesTarget::new()
                .rule_id("dog-913-009".to_string())
                .tags(
                    ApplicationSecurityExclusionFilterRulesTargetTags::new()
                        .category("attack_attempt".to_string())
                        .type_("lfi".to_string()),
                )])
            .scope(vec![ApplicationSecurityExclusionFilterScope::new()
                .env("www".to_string())
                .service("prod".to_string())]),
            ApplicationSecurityExclusionFilterType::EXCLUSION_FILTER,
        ),
    );
    let configuration = datadog::Configuration::new();
    let api = ApplicationSecurityAPI::with_config(configuration);
    let resp = api.create_application_security_exclusion_filter(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
