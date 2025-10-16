// Create a Workload Protection policy returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_csm_threats::CSMThreatsAPI;
use datadog_api_client::datadogV2::model::CloudWorkloadSecurityAgentPolicyCreateAttributes;
use datadog_api_client::datadogV2::model::CloudWorkloadSecurityAgentPolicyCreateData;
use datadog_api_client::datadogV2::model::CloudWorkloadSecurityAgentPolicyCreateRequest;
use datadog_api_client::datadogV2::model::CloudWorkloadSecurityAgentPolicyType;

#[tokio::main]
async fn main() {
    let body = CloudWorkloadSecurityAgentPolicyCreateRequest::new(
        CloudWorkloadSecurityAgentPolicyCreateData::new(
            CloudWorkloadSecurityAgentPolicyCreateAttributes::new("examplecsmthreat".to_string())
                .description("My agent policy".to_string())
                .enabled(true)
                .host_tags_lists(vec![vec!["env:test".to_string()]]),
            CloudWorkloadSecurityAgentPolicyType::POLICY,
        ),
    );
    let configuration = datadog::Configuration::new();
    let api = CSMThreatsAPI::with_config(configuration);
    let resp = api.create_csm_threats_agent_policy(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
