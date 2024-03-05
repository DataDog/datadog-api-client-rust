// Update a Cloud Workload Security Agent rule returns "OK" response
use chrono::prelude::*;
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_cloud_workload_security::*;
use datadog_api_client::datadogV2::model::*;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    // there is a valid "agent_rule" in the system
    let agent_rule_data_id = std::env::var("AGENT_RULE_DATA_ID").unwrap();
    let body =
        CloudWorkloadSecurityAgentRuleUpdateRequest::new(
            CloudWorkloadSecurityAgentRuleUpdateData::new(
                CloudWorkloadSecurityAgentRuleUpdateAttributes::new()
                    .description("Test Agent rule".to_string())
                    .enabled(true)
                    .expression(r#"exec.file.name == "sh""#.to_string()),
                CloudWorkloadSecurityAgentRuleType::AGENT_RULE,
            ),
        );
    let configuration = Configuration::new();
    let api = CloudWorkloadSecurityAPI::with_config(configuration);
    let resp = api.update_cloud_workload_security_agent_rule(agent_rule_data_id, body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
