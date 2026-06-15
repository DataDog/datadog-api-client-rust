// Create a tag policy returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_tag_policies::TagPoliciesAPI;
use datadog_api_client::datadogV2::model::TagPolicyCreateAttributes;
use datadog_api_client::datadogV2::model::TagPolicyCreateData;
use datadog_api_client::datadogV2::model::TagPolicyCreateRequest;
use datadog_api_client::datadogV2::model::TagPolicyCreateType;
use datadog_api_client::datadogV2::model::TagPolicyResourceType;
use datadog_api_client::datadogV2::model::TagPolicySource;

#[tokio::main]
async fn main() {
    let body = TagPolicyCreateRequest::new(TagPolicyCreateData::new(
        TagPolicyCreateAttributes::new(
            "Service tag must be one of api or web".to_string(),
            TagPolicyCreateType::SURFACING,
            "env".to_string(),
            TagPolicySource::LOGS,
            "service".to_string(),
            vec!["api".to_string(), "web".to_string()],
        )
        .enabled(true)
        .negated(false)
        .required(true),
        TagPolicyResourceType::TAG_POLICY,
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
