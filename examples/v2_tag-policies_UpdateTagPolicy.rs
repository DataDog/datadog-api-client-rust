// Update a tag policy returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_tag_policies::TagPoliciesAPI;
use datadog_api_client::datadogV2::model::TagPolicyAttributesUpdateRequest;
use datadog_api_client::datadogV2::model::TagPolicyDataUpdateRequest;
use datadog_api_client::datadogV2::model::TagPolicyType;
use datadog_api_client::datadogV2::model::TagPolicyUpdateRequest;

#[tokio::main]
async fn main() {
    let body = TagPolicyUpdateRequest::new(TagPolicyDataUpdateRequest::new(
        TagPolicyAttributesUpdateRequest::new()
            .enabled(true)
            .negated(false)
            .policy_name("production-tags-policy".to_string())
            .required(true)
            .scope("env".to_string())
            .source("logs".to_string())
            .tag_key("service".to_string())
            .tag_value_patterns(vec!["api".to_string(), "web".to_string()]),
        TagPolicyType::TAG_POLICY,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdateTagPolicy", true);
    let api = TagPoliciesAPI::with_config(configuration);
    let resp = api.update_tag_policy("123".to_string(), body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
