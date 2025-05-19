// Update a Workload Protection policy returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_csm_threats::CSMThreatsAPI;
use datadog_api_client::datadogV2::model::CloudWorkloadSecurityAgentPolicyType;
use datadog_api_client::datadogV2::model::CloudWorkloadSecurityAgentPolicyUpdateAttributes;
use datadog_api_client::datadogV2::model::CloudWorkloadSecurityAgentPolicyUpdateData;
use datadog_api_client::datadogV2::model::CloudWorkloadSecurityAgentPolicyUpdateRequest;

#[tokio::main]
async fn main() {
    // there is a valid "policy_rc" in the system
    let policy_data_id = std::env::var("POLICY_DATA_ID").unwrap();
    let body = CloudWorkloadSecurityAgentPolicyUpdateRequest::new(
        CloudWorkloadSecurityAgentPolicyUpdateData::new(
            CloudWorkloadSecurityAgentPolicyUpdateAttributes::new()
                .description("Updated agent policy".to_string())
                .enabled(true)
                .host_tags_lists(vec![vec!["env:test".to_string()]])
                .name("updated_agent_policy".to_string()),
            CloudWorkloadSecurityAgentPolicyType::POLICY,
        )
        .id(policy_data_id.clone()),
    );
    let configuration = datadog::Configuration::new();
    let api = CSMThreatsAPI::with_config(configuration);
    let resp = api
        .update_csm_threats_agent_policy(policy_data_id.clone(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
