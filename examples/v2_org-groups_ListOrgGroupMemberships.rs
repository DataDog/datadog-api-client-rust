// List org group memberships returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_org_groups::ListOrgGroupMembershipsOptionalParams;
use datadog_api_client::datadogV2::api_org_groups::OrgGroupsAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ListOrgGroupMemberships", true);
    let api = OrgGroupsAPI::with_config(configuration);
    let resp = api
        .list_org_group_memberships(ListOrgGroupMembershipsOptionalParams::default())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
