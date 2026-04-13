// Update an org group policy returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_org_groups::OrgGroupsAPI;
use datadog_api_client::datadogV2::model::OrgGroupPolicyType;
use datadog_api_client::datadogV2::model::OrgGroupPolicyUpdateAttributes;
use datadog_api_client::datadogV2::model::OrgGroupPolicyUpdateData;
use datadog_api_client::datadogV2::model::OrgGroupPolicyUpdateRequest;
use serde_json::Value;
use std::collections::BTreeMap;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body = OrgGroupPolicyUpdateRequest::new(OrgGroupPolicyUpdateData::new(
        OrgGroupPolicyUpdateAttributes::new()
            .content(BTreeMap::from([("value".to_string(), Value::from("UTC"))])),
        Uuid::parse_str("1a2b3c4d-5e6f-7890-abcd-ef0123456789").expect("invalid UUID"),
        OrgGroupPolicyType::ORG_GROUP_POLICIES,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdateOrgGroupPolicy", true);
    let api = OrgGroupsAPI::with_config(configuration);
    let resp = api
        .update_org_group_policy(
            Uuid::parse_str("1a2b3c4d-5e6f-7890-abcd-ef0123456789").expect("invalid UUID"),
            body,
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
