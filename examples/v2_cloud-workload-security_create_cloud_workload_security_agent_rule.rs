// Create a Cloud Workload Security Agent rule returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_cloud_workload_security::*;
use datadog_api_client::datadogV2::model::*;

#[tokio::main]
async fn main() {
    let body = CloudWorkloadSecurityAgentRuleCreateRequest::new(
        CloudWorkloadSecurityAgentRuleCreateData::new(
            CloudWorkloadSecurityAgentRuleCreateAttributes::new(
                r#"exec.file.name == "sh""#.to_string(),
                "examplecloudworkloadsecurity".to_string(),
            )
            .description("Test Agent rule".to_string())
            .enabled(true),
            CloudWorkloadSecurityAgentRuleType::AGENT_RULE,
        ),
    );
    let configuration = Configuration::new();
    let api = CloudWorkloadSecurityAPI::with_config(configuration);
    let resp = api.create_cloud_workload_security_agent_rule(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
