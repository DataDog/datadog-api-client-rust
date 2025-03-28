// Batch update CSM Threats Agent policies returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_csm_threats::CSMThreatsAPI;
use datadog_api_client::datadogV2::model::CloudWorkloadSecurityAgentPolicyBatchUpdateAttributes;
use datadog_api_client::datadogV2::model::CloudWorkloadSecurityAgentPolicyBatchUpdateAttributesPoliciesItems;
use datadog_api_client::datadogV2::model::CloudWorkloadSecurityAgentPolicyBatchUpdateData;
use datadog_api_client::datadogV2::model::CloudWorkloadSecurityAgentPolicyBatchUpdateDataType;
use datadog_api_client::datadogV2::model::CloudWorkloadSecurityAgentPolicyBatchUpdateRequest;

#[tokio::main]
async fn main() {
    // there is a valid "policy_rc" in the system
    let policy_data_id = std::env::var("POLICY_DATA_ID").unwrap();
    let body = CloudWorkloadSecurityAgentPolicyBatchUpdateRequest::new(
        CloudWorkloadSecurityAgentPolicyBatchUpdateData::new(
            CloudWorkloadSecurityAgentPolicyBatchUpdateAttributes::new().policies(vec![
                CloudWorkloadSecurityAgentPolicyBatchUpdateAttributesPoliciesItems::new()
                    .delete(false)
                    .description("Updated agent policy".to_string())
                    .enabled(true)
                    .host_tags(vec!["env:test".to_string()])
                    .id(policy_data_id.clone())
                    .name("updated_agent_policy".to_string())
                    .priority(20),
            ]),
            "batch_update_req".to_string(),
            CloudWorkloadSecurityAgentPolicyBatchUpdateDataType::POLICIES,
        ),
    );
    let configuration = datadog::Configuration::new();
    let api = CSMThreatsAPI::with_config(configuration);
    let resp = api.batch_update_csm_threats_agent_policy(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
