// Create a tag policy returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_tag_policies::TagPoliciesAPI;
use datadog_api_client::datadogV2::model::TagPolicyAttributesRequest;
use datadog_api_client::datadogV2::model::TagPolicyCreateRequest;
use datadog_api_client::datadogV2::model::TagPolicyDataRequest;
use datadog_api_client::datadogV2::model::TagPolicyType;

#[tokio::main]
async fn main() {
    let body = TagPolicyCreateRequest::new(TagPolicyDataRequest::new(
        TagPolicyAttributesRequest::new(
            true,
            false,
            "production-tags-policy".to_string(),
            true,
            "env".to_string(),
            "logs".to_string(),
            "service".to_string(),
            vec!["api".to_string(), "web".to_string()],
        ),
        TagPolicyType::TAG_POLICY,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateTagPolicy", true);
    let api = TagPoliciesAPI::with_config(configuration);
    let resp = api.create_tag_policy(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
