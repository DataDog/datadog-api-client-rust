// Update an execution policy to Slack with scope and targets returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_execution_policy::ExecutionPolicyAPI;
use datadog_api_client::datadogV2::model::ExecutionPolicyActionPattern;
use datadog_api_client::datadogV2::model::ExecutionPolicyEffect;
use datadog_api_client::datadogV2::model::ExecutionPolicyIntegration;
use datadog_api_client::datadogV2::model::ExecutionPolicyScope;
use datadog_api_client::datadogV2::model::ExecutionPolicySlackScope;
use datadog_api_client::datadogV2::model::ExecutionPolicySlackScopeRule;
use datadog_api_client::datadogV2::model::ExecutionPolicyTarget;
use datadog_api_client::datadogV2::model::ExecutionPolicyType;
use datadog_api_client::datadogV2::model::ExecutionPolicyUpdateRequest;
use datadog_api_client::datadogV2::model::ExecutionPolicyUpdateRequestData;
use datadog_api_client::datadogV2::model::ExecutionPolicyWriteAttributes;

#[tokio::main]
async fn main() {
    // there is a valid "execution_policy" in the system
    let execution_policy_data_id = std::env::var("EXECUTION_POLICY_DATA_ID").unwrap();
    let body = ExecutionPolicyUpdateRequest::new(ExecutionPolicyUpdateRequestData::new(
        ExecutionPolicyWriteAttributes::new(
            ExecutionPolicyActionPattern::new(
                vec!["*".to_string()],
                ExecutionPolicyIntegration::INTEGRATION_SLACK,
            ),
            ExecutionPolicyEffect::DENY,
            "Cassette Execution Policy Updated".to_string(),
        )
        .scope(
            ExecutionPolicyScope::new().slack(ExecutionPolicySlackScope::new(vec![
                ExecutionPolicySlackScopeRule::new(vec!["incident-denied".to_string()]),
            ])),
        )
        .targets(vec![ExecutionPolicyTarget::new(vec![
            "slack_workspace_id:T01234567".to_string(),
        ])
        .name(Some("Incident workspace".to_string()))]),
        execution_policy_data_id.clone(),
        ExecutionPolicyType::EXECUTION_POLICY,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.update_execution_policy", true);
    let api = ExecutionPolicyAPI::with_config(configuration);
    let resp = api
        .update_execution_policy(execution_policy_data_id.clone(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
