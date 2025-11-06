// Update a WAF Policy returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_application_security::ApplicationSecurityAPI;
use datadog_api_client::datadogV2::model::ApplicationSecurityPolicyRuleOverride;
use datadog_api_client::datadogV2::model::ApplicationSecurityPolicyScope;
use datadog_api_client::datadogV2::model::ApplicationSecurityPolicyType;
use datadog_api_client::datadogV2::model::ApplicationSecurityPolicyUpdateAttributes;
use datadog_api_client::datadogV2::model::ApplicationSecurityPolicyUpdateData;
use datadog_api_client::datadogV2::model::ApplicationSecurityPolicyUpdateRequest;

#[tokio::main]
async fn main() {
    let body =
        ApplicationSecurityPolicyUpdateRequest::new(ApplicationSecurityPolicyUpdateData::new(
            ApplicationSecurityPolicyUpdateAttributes::new(
                "Policy applied to internal web applications.".to_string(),
                false,
                "Internal Network Policy".to_string(),
                vec!["attack-tools".to_string()],
                vec![ApplicationSecurityPolicyRuleOverride::new(
                    false,
                    true,
                    "rasp-001-002".to_string(),
                )],
                vec![ApplicationSecurityPolicyScope::new(
                    "prod".to_string(),
                    "billing-service".to_string(),
                )],
                0,
            ),
            ApplicationSecurityPolicyType::POLICY,
        ));
    let configuration = datadog::Configuration::new();
    let api = ApplicationSecurityAPI::with_config(configuration);
    let resp = api
        .update_application_security_waf_policy("policy_id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
