// Bulk update org group memberships returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_org_groups::OrgGroupsAPI;
use datadog_api_client::datadogV2::model::GlobalOrgIdentifier;
use datadog_api_client::datadogV2::model::OrgGroupMembershipBulkUpdateAttributes;
use datadog_api_client::datadogV2::model::OrgGroupMembershipBulkUpdateData;
use datadog_api_client::datadogV2::model::OrgGroupMembershipBulkUpdateRelationships;
use datadog_api_client::datadogV2::model::OrgGroupMembershipBulkUpdateRequest;
use datadog_api_client::datadogV2::model::OrgGroupMembershipBulkUpdateType;
use datadog_api_client::datadogV2::model::OrgGroupRelationshipToOne;
use datadog_api_client::datadogV2::model::OrgGroupRelationshipToOneData;
use datadog_api_client::datadogV2::model::OrgGroupType;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body = OrgGroupMembershipBulkUpdateRequest::new(OrgGroupMembershipBulkUpdateData::new(
        OrgGroupMembershipBulkUpdateAttributes::new(vec![GlobalOrgIdentifier::new(
            "datadoghq.com".to_string(),
            Uuid::parse_str("c3d4e5f6-a7b8-9012-cdef-012345678901").expect("invalid UUID"),
        )]),
        OrgGroupMembershipBulkUpdateRelationships::new(
            OrgGroupRelationshipToOne::new(OrgGroupRelationshipToOneData::new(
                Uuid::parse_str("a1b2c3d4-e5f6-7890-abcd-ef0123456789").expect("invalid UUID"),
                OrgGroupType::ORG_GROUPS,
            )),
            OrgGroupRelationshipToOne::new(OrgGroupRelationshipToOneData::new(
                Uuid::parse_str("a1b2c3d4-e5f6-7890-abcd-ef0123456789").expect("invalid UUID"),
                OrgGroupType::ORG_GROUPS,
            )),
        ),
        OrgGroupMembershipBulkUpdateType::ORG_GROUP_MEMBERSHIP_BULK_UPDATES,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.BulkUpdateOrgGroupMemberships", true);
    let api = OrgGroupsAPI::with_config(configuration);
    let resp = api.bulk_update_org_group_memberships(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
