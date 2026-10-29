// Create a Workload Protection agent rule (US1-FED) returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_csm_threats::CSMThreatsAPI;
use datadog_api_client::datadogV2::model::CloudWorkloadSecurityAgentRuleAction;
use datadog_api_client::datadogV2::model::CloudWorkloadSecurityAgentRuleActionCoreDump;
use datadog_api_client::datadogV2::model::CloudWorkloadSecurityAgentRuleActionHash;
use datadog_api_client::datadogV2::model::CloudWorkloadSecurityAgentRuleActionLog;
use datadog_api_client::datadogV2::model::CloudWorkloadSecurityAgentRuleActionMetadata;
use datadog_api_client::datadogV2::model::CloudWorkloadSecurityAgentRuleActionNetworkFilter;
use datadog_api_client::datadogV2::model::CloudWorkloadSecurityAgentRuleActionSet;
use datadog_api_client::datadogV2::model::CloudWorkloadSecurityAgentRuleCreateAttributes;
use datadog_api_client::datadogV2::model::CloudWorkloadSecurityAgentRuleCreateData;
use datadog_api_client::datadogV2::model::CloudWorkloadSecurityAgentRuleCreateRequest;
use datadog_api_client::datadogV2::model::CloudWorkloadSecurityAgentRuleKill;
use datadog_api_client::datadogV2::model::CloudWorkloadSecurityAgentRuleType;

#[tokio::main]
async fn main() {
    let body = CloudWorkloadSecurityAgentRuleCreateRequest::new(
        CloudWorkloadSecurityAgentRuleCreateData::new(
            CloudWorkloadSecurityAgentRuleCreateAttributes::new(
                r#"exec.file.name == "sh""#.to_string(),
                "my_agent_rule".to_string(),
            )
            .actions(Some(vec![CloudWorkloadSecurityAgentRuleAction::new()
                .coredump(CloudWorkloadSecurityAgentRuleActionCoreDump::new())
                .disabled(false)
                .hash(CloudWorkloadSecurityAgentRuleActionHash::new())
                .kill(CloudWorkloadSecurityAgentRuleKill::new())
                .log(CloudWorkloadSecurityAgentRuleActionLog::new())
                .metadata(CloudWorkloadSecurityAgentRuleActionMetadata::new())
                .network_filter(CloudWorkloadSecurityAgentRuleActionNetworkFilter::new())
                .set(CloudWorkloadSecurityAgentRuleActionSet::new())]))
            .blocking(vec![])
            .description("My Agent rule".to_string())
            .disabled(vec![])
            .enabled(true)
            .filters(vec![])
            .monitoring(vec![])
            .policy_id("a8c8e364-6556-434d-b798-a4c23de29c0b".to_string())
            .product_tags(vec![])
            .silent(false),
            CloudWorkloadSecurityAgentRuleType::AGENT_RULE,
        ),
    );
    let configuration = datadog::Configuration::new();
    let api = CSMThreatsAPI::with_config(configuration);
    let resp = api.create_cloud_workload_security_agent_rule(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
