// Update an org group policy override returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_org_groups::OrgGroupsAPI;
use datadog_api_client::datadogV2::model::OrgGroupPolicyOverrideType;
use datadog_api_client::datadogV2::model::OrgGroupPolicyOverrideUpdateAttributes;
use datadog_api_client::datadogV2::model::OrgGroupPolicyOverrideUpdateData;
use datadog_api_client::datadogV2::model::OrgGroupPolicyOverrideUpdateRequest;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body = OrgGroupPolicyOverrideUpdateRequest::new(OrgGroupPolicyOverrideUpdateData::new(
        OrgGroupPolicyOverrideUpdateAttributes::new(
            "datadoghq.com".to_string(),
            Uuid::parse_str("c3d4e5f6-a7b8-9012-cdef-012345678901").expect("invalid UUID"),
        ),
        Uuid::parse_str("9f8e7d6c-5b4a-3210-fedc-ba0987654321").expect("invalid UUID"),
        OrgGroupPolicyOverrideType::ORG_GROUP_POLICY_OVERRIDES,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdateOrgGroupPolicyOverride", true);
    let api = OrgGroupsAPI::with_config(configuration);
    let resp = api
        .update_org_group_policy_override(
            Uuid::parse_str("9f8e7d6c-5b4a-3210-fedc-ba0987654321").expect("invalid UUID"),
            body,
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
