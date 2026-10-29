// Update a Workload Protection agent rule (US1-FED) returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_csm_threats::CSMThreatsAPI;
use datadog_api_client::datadogV2::model::CloudWorkloadSecurityAgentRuleAction;
use datadog_api_client::datadogV2::model::CloudWorkloadSecurityAgentRuleActionCoreDump;
use datadog_api_client::datadogV2::model::CloudWorkloadSecurityAgentRuleActionHash;
use datadog_api_client::datadogV2::model::CloudWorkloadSecurityAgentRuleActionLog;
use datadog_api_client::datadogV2::model::CloudWorkloadSecurityAgentRuleActionMetadata;
use datadog_api_client::datadogV2::model::CloudWorkloadSecurityAgentRuleActionNetworkFilter;
use datadog_api_client::datadogV2::model::CloudWorkloadSecurityAgentRuleActionSet;
use datadog_api_client::datadogV2::model::CloudWorkloadSecurityAgentRuleKill;
use datadog_api_client::datadogV2::model::CloudWorkloadSecurityAgentRuleType;
use datadog_api_client::datadogV2::model::CloudWorkloadSecurityAgentRuleUpdateAttributes;
use datadog_api_client::datadogV2::model::CloudWorkloadSecurityAgentRuleUpdateData;
use datadog_api_client::datadogV2::model::CloudWorkloadSecurityAgentRuleUpdateRequest;

#[tokio::main]
async fn main() {
    let body = CloudWorkloadSecurityAgentRuleUpdateRequest::new(
        CloudWorkloadSecurityAgentRuleUpdateData::new(
            CloudWorkloadSecurityAgentRuleUpdateAttributes::new()
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
                .expression(r#"exec.file.name == "sh""#.to_string())
                .monitoring(vec![])
                .policy_id("a8c8e364-6556-434d-b798-a4c23de29c0b".to_string())
                .product_tags(vec![])
                .silent(false),
            CloudWorkloadSecurityAgentRuleType::AGENT_RULE,
        )
        .id("3dd-0uc-h1s".to_string()),
    );
    let configuration = datadog::Configuration::new();
    let api = CSMThreatsAPI::with_config(configuration);
    let resp = api
        .update_cloud_workload_security_agent_rule("agent_rule_id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
