// Create a Workload Protection agent rule with set action with expression returns
// "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_csm_threats::CSMThreatsAPI;
use datadog_api_client::datadogV2::model::CloudWorkloadSecurityAgentRuleAction;
use datadog_api_client::datadogV2::model::CloudWorkloadSecurityAgentRuleActionSet;
use datadog_api_client::datadogV2::model::CloudWorkloadSecurityAgentRuleCreateAttributes;
use datadog_api_client::datadogV2::model::CloudWorkloadSecurityAgentRuleCreateData;
use datadog_api_client::datadogV2::model::CloudWorkloadSecurityAgentRuleCreateRequest;
use datadog_api_client::datadogV2::model::CloudWorkloadSecurityAgentRuleType;

#[tokio::main]
async fn main() {
    // there is a valid "policy_rc" in the system
    let policy_data_id = std::env::var("POLICY_DATA_ID").unwrap();
    let body = CloudWorkloadSecurityAgentRuleCreateRequest::new(
        CloudWorkloadSecurityAgentRuleCreateData::new(
            CloudWorkloadSecurityAgentRuleCreateAttributes::new(
                r#"exec.file.name == "sh""#.to_string(),
                "examplecsmthreat".to_string(),
            )
            .actions(Some(vec![CloudWorkloadSecurityAgentRuleAction::new().set(
                CloudWorkloadSecurityAgentRuleActionSet::new()
                    .default_value("/dev/null".to_string())
                    .expression("exec.file.path".to_string())
                    .name("test_set".to_string())
                    .scope("process".to_string()),
            )]))
            .description("My Agent rule with set action with expression".to_string())
            .enabled(true)
            .filters(vec![])
            .policy_id(policy_data_id.clone())
            .product_tags(vec![]),
            CloudWorkloadSecurityAgentRuleType::AGENT_RULE,
        ),
    );
    let configuration = datadog::Configuration::new();
    let api = CSMThreatsAPI::with_config(configuration);
    let resp = api.create_csm_threats_agent_rule(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
