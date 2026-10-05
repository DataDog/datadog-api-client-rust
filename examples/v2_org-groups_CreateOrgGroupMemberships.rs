// Create org group memberships returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_org_groups::OrgGroupsAPI;
use datadog_api_client::datadogV2::model::GlobalOrgIdentifier;
use datadog_api_client::datadogV2::model::OrgGroupMembershipCreateAttributes;
use datadog_api_client::datadogV2::model::OrgGroupMembershipCreateData;
use datadog_api_client::datadogV2::model::OrgGroupMembershipCreateRelationships;
use datadog_api_client::datadogV2::model::OrgGroupMembershipCreateRequest;
use datadog_api_client::datadogV2::model::OrgGroupMembershipType;
use datadog_api_client::datadogV2::model::OrgGroupRelationshipToOne;
use datadog_api_client::datadogV2::model::OrgGroupRelationshipToOneData;
use datadog_api_client::datadogV2::model::OrgGroupType;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body = OrgGroupMembershipCreateRequest::new(OrgGroupMembershipCreateData::new(
        OrgGroupMembershipCreateAttributes::new(vec![GlobalOrgIdentifier::new(
            "us1".to_string(),
            Uuid::parse_str("c3d4e5f6-a7b8-9012-cdef-012345678901").expect("invalid UUID"),
        )]),
        OrgGroupMembershipCreateRelationships::new(OrgGroupRelationshipToOne::new(
            OrgGroupRelationshipToOneData::new(
                Uuid::parse_str("a1b2c3d4-e5f6-7890-abcd-ef0123456789").expect("invalid UUID"),
                OrgGroupType::ORG_GROUPS,
            ),
        )),
        OrgGroupMembershipType::ORG_GROUP_MEMBERSHIPS,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateOrgGroupMemberships", true);
    let api = OrgGroupsAPI::with_config(configuration);
    let resp = api.create_org_group_memberships(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
