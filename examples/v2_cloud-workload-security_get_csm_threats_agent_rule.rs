// Get a CSM Threats Agent rule returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_cloud_workload_security::CloudWorkloadSecurityAPI;

#[tokio::main]
async fn main() {
    // there is a valid "agent_rule_rc" in the system
    let agent_rule_data_id = std::env::var("AGENT_RULE_DATA_ID").unwrap();
    let configuration = Configuration::new();
    let api = CloudWorkloadSecurityAPI::with_config(configuration);
    let resp = api
        .get_csm_threats_agent_rule(agent_rule_data_id.clone())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
