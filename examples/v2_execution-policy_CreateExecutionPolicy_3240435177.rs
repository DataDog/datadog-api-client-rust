// Create a Slack execution policy with scope and targets returns "Created"
// response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_execution_policy::ExecutionPolicyAPI;
use datadog_api_client::datadogV2::model::ExecutionPolicyActionPattern;
use datadog_api_client::datadogV2::model::ExecutionPolicyCreateRequest;
use datadog_api_client::datadogV2::model::ExecutionPolicyCreateRequestData;
use datadog_api_client::datadogV2::model::ExecutionPolicyEffect;
use datadog_api_client::datadogV2::model::ExecutionPolicyIntegration;
use datadog_api_client::datadogV2::model::ExecutionPolicyScope;
use datadog_api_client::datadogV2::model::ExecutionPolicySlackScope;
use datadog_api_client::datadogV2::model::ExecutionPolicySlackScopeRule;
use datadog_api_client::datadogV2::model::ExecutionPolicyTarget;
use datadog_api_client::datadogV2::model::ExecutionPolicyType;
use datadog_api_client::datadogV2::model::ExecutionPolicyWriteAttributes;

#[tokio::main]
async fn main() {
    let body = ExecutionPolicyCreateRequest::new(ExecutionPolicyCreateRequestData::new(
        ExecutionPolicyWriteAttributes::new(
            ExecutionPolicyActionPattern::new(
                vec!["com.datadoghq.slack.*".to_string()],
                ExecutionPolicyIntegration::INTEGRATION_SLACK,
            ),
            ExecutionPolicyEffect::ALLOW,
            "Cassette Execution Policy exampleexecutionpolicy".to_string(),
        )
        .scope(
            ExecutionPolicyScope::new().slack(ExecutionPolicySlackScope::new(vec![
                ExecutionPolicySlackScopeRule::new(vec!["incident-*".to_string()]),
            ])),
        )
        .targets(vec![ExecutionPolicyTarget::new(vec![
            "slack_workspace_id:T01234567".to_string(),
        ])
        .name(Some("Incident workspace".to_string()))]),
        ExecutionPolicyType::EXECUTION_POLICY,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.create_execution_policy", true);
    let api = ExecutionPolicyAPI::with_config(configuration);
    let resp = api.create_execution_policy(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
