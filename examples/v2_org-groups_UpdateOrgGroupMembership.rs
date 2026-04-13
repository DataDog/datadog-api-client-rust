// Update an org group membership returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_org_groups::OrgGroupsAPI;
use datadog_api_client::datadogV2::model::OrgGroupMembershipType;
use datadog_api_client::datadogV2::model::OrgGroupMembershipUpdateData;
use datadog_api_client::datadogV2::model::OrgGroupMembershipUpdateRelationships;
use datadog_api_client::datadogV2::model::OrgGroupMembershipUpdateRequest;
use datadog_api_client::datadogV2::model::OrgGroupRelationshipToOne;
use datadog_api_client::datadogV2::model::OrgGroupRelationshipToOneData;
use datadog_api_client::datadogV2::model::OrgGroupType;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body = OrgGroupMembershipUpdateRequest::new(OrgGroupMembershipUpdateData::new(
        Uuid::parse_str("f1e2d3c4-b5a6-7890-1234-567890abcdef").expect("invalid UUID"),
        OrgGroupMembershipUpdateRelationships::new(OrgGroupRelationshipToOne::new(
            OrgGroupRelationshipToOneData::new(
                Uuid::parse_str("a1b2c3d4-e5f6-7890-abcd-ef0123456789").expect("invalid UUID"),
                OrgGroupType::ORG_GROUPS,
            ),
        )),
        OrgGroupMembershipType::ORG_GROUP_MEMBERSHIPS,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdateOrgGroupMembership", true);
    let api = OrgGroupsAPI::with_config(configuration);
    let resp = api
        .update_org_group_membership(
            Uuid::parse_str("f1e2d3c4-b5a6-7890-1234-567890abcdef").expect("invalid UUID"),
            body,
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
