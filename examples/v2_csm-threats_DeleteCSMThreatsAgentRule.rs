// Delete a Workload Protection agent rule returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_csm_threats::CSMThreatsAPI;
use datadog_api_client::datadogV2::api_csm_threats::DeleteCSMThreatsAgentRuleOptionalParams;

#[tokio::main]
async fn main() {
    // there is a valid "agent_rule_rc" in the system
    let agent_rule_data_id = std::env::var("AGENT_RULE_DATA_ID").unwrap();

    // there is a valid "policy_rc" in the system
    let policy_data_id = std::env::var("POLICY_DATA_ID").unwrap();
    let configuration = datadog::Configuration::new();
    let api = CSMThreatsAPI::with_config(configuration);
    let resp = api
        .delete_csm_threats_agent_rule(
            agent_rule_data_id.clone(),
            DeleteCSMThreatsAgentRuleOptionalParams::default().policy_id(policy_data_id.clone()),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
