// Create an org group policy override returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_org_groups::OrgGroupsAPI;
use datadog_api_client::datadogV2::model::OrgGroupPolicyOverrideCreateAttributes;
use datadog_api_client::datadogV2::model::OrgGroupPolicyOverrideCreateData;
use datadog_api_client::datadogV2::model::OrgGroupPolicyOverrideCreateRelationships;
use datadog_api_client::datadogV2::model::OrgGroupPolicyOverrideCreateRequest;
use datadog_api_client::datadogV2::model::OrgGroupPolicyOverrideType;
use datadog_api_client::datadogV2::model::OrgGroupPolicyRelationshipToOne;
use datadog_api_client::datadogV2::model::OrgGroupPolicyRelationshipToOneData;
use datadog_api_client::datadogV2::model::OrgGroupPolicyType;
use datadog_api_client::datadogV2::model::OrgGroupRelationshipToOne;
use datadog_api_client::datadogV2::model::OrgGroupRelationshipToOneData;
use datadog_api_client::datadogV2::model::OrgGroupType;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body = OrgGroupPolicyOverrideCreateRequest::new(OrgGroupPolicyOverrideCreateData::new(
        OrgGroupPolicyOverrideCreateAttributes::new(
            "datadoghq.com".to_string(),
            Uuid::parse_str("c3d4e5f6-a7b8-9012-cdef-012345678901").expect("invalid UUID"),
        ),
        OrgGroupPolicyOverrideCreateRelationships::new(
            OrgGroupRelationshipToOne::new(OrgGroupRelationshipToOneData::new(
                Uuid::parse_str("a1b2c3d4-e5f6-7890-abcd-ef0123456789").expect("invalid UUID"),
                OrgGroupType::ORG_GROUPS,
            )),
            OrgGroupPolicyRelationshipToOne::new(OrgGroupPolicyRelationshipToOneData::new(
                Uuid::parse_str("1a2b3c4d-5e6f-7890-abcd-ef0123456789").expect("invalid UUID"),
                OrgGroupPolicyType::ORG_GROUP_POLICIES,
            )),
        ),
        OrgGroupPolicyOverrideType::ORG_GROUP_POLICY_OVERRIDES,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateOrgGroupPolicyOverride", true);
    let api = OrgGroupsAPI::with_config(configuration);
    let resp = api.create_org_group_policy_override(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
