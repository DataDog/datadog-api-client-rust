// Update an org group returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_org_groups::OrgGroupsAPI;
use datadog_api_client::datadogV2::model::OrgGroupType;
use datadog_api_client::datadogV2::model::OrgGroupUpdateAttributes;
use datadog_api_client::datadogV2::model::OrgGroupUpdateData;
use datadog_api_client::datadogV2::model::OrgGroupUpdateRequest;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body = OrgGroupUpdateRequest::new(OrgGroupUpdateData::new(
        OrgGroupUpdateAttributes::new("Updated Org Group Name".to_string()),
        Uuid::parse_str("a1b2c3d4-e5f6-7890-abcd-ef0123456789").expect("invalid UUID"),
        OrgGroupType::ORG_GROUPS,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdateOrgGroup", true);
    let api = OrgGroupsAPI::with_config(configuration);
    let resp = api
        .update_org_group(
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
