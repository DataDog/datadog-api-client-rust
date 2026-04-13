// Create an org group returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_org_groups::OrgGroupsAPI;
use datadog_api_client::datadogV2::model::OrgGroupCreateAttributes;
use datadog_api_client::datadogV2::model::OrgGroupCreateData;
use datadog_api_client::datadogV2::model::OrgGroupCreateRequest;
use datadog_api_client::datadogV2::model::OrgGroupType;

#[tokio::main]
async fn main() {
    let body = OrgGroupCreateRequest::new(OrgGroupCreateData::new(
        OrgGroupCreateAttributes::new("My Org Group".to_string()),
        OrgGroupType::ORG_GROUPS,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateOrgGroup", true);
    let api = OrgGroupsAPI::with_config(configuration);
    let resp = api.create_org_group(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
