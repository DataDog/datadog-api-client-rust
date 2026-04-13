// List org group policies returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_org_groups::ListOrgGroupPoliciesOptionalParams;
use datadog_api_client::datadogV2::api_org_groups::OrgGroupsAPI;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ListOrgGroupPolicies", true);
    let api = OrgGroupsAPI::with_config(configuration);
    let resp = api
        .list_org_group_policies(
            Uuid::parse_str("a1b2c3d4-e5f6-7890-abcd-ef0123456789").expect("invalid UUID"),
            ListOrgGroupPoliciesOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
