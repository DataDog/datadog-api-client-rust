// Get an org group membership returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_org_groups::OrgGroupsAPI;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.GetOrgGroupMembership", true);
    let api = OrgGroupsAPI::with_config(configuration);
    let resp = api
        .get_org_group_membership(
            Uuid::parse_str("f1e2d3c4-b5a6-7890-1234-567890abcdef").expect("invalid UUID"),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
