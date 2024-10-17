// Create an ASM Exclusion filter returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_application_security_exclusion_filters::ApplicationSecurityExclusionFiltersAPI;
use datadog_api_client::datadogV2::model::ApplicationSecurityExclusionFilterCreateAttributes;
use datadog_api_client::datadogV2::model::ApplicationSecurityExclusionFilterCreateData;
use datadog_api_client::datadogV2::model::ApplicationSecurityExclusionFilterCreateRequest;
use datadog_api_client::datadogV2::model::ApplicationSecurityExclusionFilterRulesTarget;
use datadog_api_client::datadogV2::model::ApplicationSecurityExclusionFilterScope;
use datadog_api_client::datadogV2::model::ApplicationSecurityExclusionFilterType;

#[tokio::main]
async fn main() {
    let body = ApplicationSecurityExclusionFilterCreateRequest::new(
        ApplicationSecurityExclusionFilterCreateData::new(
            ApplicationSecurityExclusionFilterCreateAttributes::new(
                "my description".to_string(),
                true,
                "*".to_string(),
            )
            .rules_target(vec![ApplicationSecurityExclusionFilterRulesTarget::new()])
            .scope(vec![ApplicationSecurityExclusionFilterScope::new()
                .env("staging".to_string())
                .service("container-resolver".to_string())]),
            ApplicationSecurityExclusionFilterType::EXCLUSION_FILTER,
        ),
    );
    let configuration = datadog::Configuration::new();
    let api = ApplicationSecurityExclusionFiltersAPI::with_config(configuration);
    let resp = api.create_application_security_exclusion_filter(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
