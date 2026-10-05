// Bulk delete org group memberships returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_org_groups::OrgGroupsAPI;
use datadog_api_client::datadogV2::model::OrgGroupMembershipBulkDeleteRequest;
use datadog_api_client::datadogV2::model::OrgGroupMembershipBulkDeleteRequestData;
use datadog_api_client::datadogV2::model::OrgGroupMembershipType;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body = OrgGroupMembershipBulkDeleteRequest::new(vec![
        OrgGroupMembershipBulkDeleteRequestData::new(
            Uuid::parse_str("f1e2d3c4-b5a6-7890-1234-567890abcdef").expect("invalid UUID"),
            OrgGroupMembershipType::ORG_GROUP_MEMBERSHIPS,
        ),
    ]);
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.BulkDeleteOrgGroupMemberships", true);
    let api = OrgGroupsAPI::with_config(configuration);
    let resp = api
        .bulk_delete_org_group_memberships(
            Uuid::parse_str("a1b2c3d4-e5f6-7890-abcd-ef0123456789").expect("invalid UUID"),
            body,
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
