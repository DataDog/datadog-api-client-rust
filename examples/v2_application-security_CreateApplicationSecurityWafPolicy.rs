// Create a WAF Policy returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_application_security::ApplicationSecurityAPI;
use datadog_api_client::datadogV2::model::ApplicationSecurityPolicyCreateAttributes;
use datadog_api_client::datadogV2::model::ApplicationSecurityPolicyCreateData;
use datadog_api_client::datadogV2::model::ApplicationSecurityPolicyCreateRequest;
use datadog_api_client::datadogV2::model::ApplicationSecurityPolicyRuleOverride;
use datadog_api_client::datadogV2::model::ApplicationSecurityPolicyScope;
use datadog_api_client::datadogV2::model::ApplicationSecurityPolicyType;

#[tokio::main]
async fn main() {
    let body =
        ApplicationSecurityPolicyCreateRequest::new(ApplicationSecurityPolicyCreateData::new(
            ApplicationSecurityPolicyCreateAttributes::new(
                "recommended".to_string(),
                "Policy applied to internal web applications.".to_string(),
                "Internal Network Policy".to_string(),
            )
            .is_default(false)
            .protection_presets(vec!["attack-tools".to_string()])
            .rules(vec![ApplicationSecurityPolicyRuleOverride::new(
                false,
                true,
                "rasp-001-002".to_string(),
            )])
            .scope(vec![ApplicationSecurityPolicyScope::new(
                "prod".to_string(),
                "billing-service".to_string(),
            )])
            .version(0),
            ApplicationSecurityPolicyType::POLICY,
        ));
    let configuration = datadog::Configuration::new();
    let api = ApplicationSecurityAPI::with_config(configuration);
    let resp = api.create_application_security_waf_policy(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
