// Delete an org group policy override returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_org_groups::OrgGroupsAPI;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.DeleteOrgGroupPolicyOverride", true);
    let api = OrgGroupsAPI::with_config(configuration);
    let resp = api
        .delete_org_group_policy_override(
            Uuid::parse_str("9f8e7d6c-5b4a-3210-fedc-ba0987654321").expect("invalid UUID"),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
