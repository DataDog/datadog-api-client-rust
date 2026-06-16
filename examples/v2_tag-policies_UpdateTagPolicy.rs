// Update a tag policy returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_tag_policies::TagPoliciesAPI;
use datadog_api_client::datadogV2::model::TagPolicyResourceType;
use datadog_api_client::datadogV2::model::TagPolicyType;
use datadog_api_client::datadogV2::model::TagPolicyUpdateAttributes;
use datadog_api_client::datadogV2::model::TagPolicyUpdateData;
use datadog_api_client::datadogV2::model::TagPolicyUpdateRequest;

#[tokio::main]
async fn main() {
    let body = TagPolicyUpdateRequest::new(
        TagPolicyUpdateData::new("123".to_string(), TagPolicyResourceType::TAG_POLICY).attributes(
            TagPolicyUpdateAttributes::new()
                .policy_type(TagPolicyType::SURFACING)
                .tag_value_patterns(vec![]),
        ),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdateTagPolicy", true);
    let api = TagPoliciesAPI::with_config(configuration);
    let resp = api.update_tag_policy("policy_id".to_string(), body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
