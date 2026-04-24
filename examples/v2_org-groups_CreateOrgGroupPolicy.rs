// Create an org group policy returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_org_groups::OrgGroupsAPI;
use datadog_api_client::datadogV2::model::OrgGroupPolicyCreateAttributes;
use datadog_api_client::datadogV2::model::OrgGroupPolicyCreateData;
use datadog_api_client::datadogV2::model::OrgGroupPolicyCreateRelationships;
use datadog_api_client::datadogV2::model::OrgGroupPolicyCreateRequest;
use datadog_api_client::datadogV2::model::OrgGroupPolicyEnforcementTier;
use datadog_api_client::datadogV2::model::OrgGroupPolicyPolicyType;
use datadog_api_client::datadogV2::model::OrgGroupPolicyType;
use datadog_api_client::datadogV2::model::OrgGroupRelationshipToOne;
use datadog_api_client::datadogV2::model::OrgGroupRelationshipToOneData;
use datadog_api_client::datadogV2::model::OrgGroupType;
use serde_json::Value;
use std::collections::BTreeMap;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body = OrgGroupPolicyCreateRequest::new(OrgGroupPolicyCreateData::new(
        OrgGroupPolicyCreateAttributes::new(
            BTreeMap::from([("value".to_string(), Value::from("UTC"))]),
            "monitor_timezone".to_string(),
        )
        .enforcement_tier(OrgGroupPolicyEnforcementTier::DEFAULT)
        .policy_type(OrgGroupPolicyPolicyType::ORG_CONFIG),
        OrgGroupPolicyCreateRelationships::new(OrgGroupRelationshipToOne::new(
            OrgGroupRelationshipToOneData::new(
                Uuid::parse_str("a1b2c3d4-e5f6-7890-abcd-ef0123456789").expect("invalid UUID"),
                OrgGroupType::ORG_GROUPS,
            ),
        )),
        OrgGroupPolicyType::ORG_GROUP_POLICIES,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateOrgGroupPolicy", true);
    let api = OrgGroupsAPI::with_config(configuration);
    let resp = api.create_org_group_policy(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
