// Create an execution policy returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_execution_policy::ExecutionPolicyAPI;
use datadog_api_client::datadogV2::model::ExecutionPolicyActionPattern;
use datadog_api_client::datadogV2::model::ExecutionPolicyCreateRequest;
use datadog_api_client::datadogV2::model::ExecutionPolicyCreateRequestData;
use datadog_api_client::datadogV2::model::ExecutionPolicyEffect;
use datadog_api_client::datadogV2::model::ExecutionPolicyIntegration;
use datadog_api_client::datadogV2::model::ExecutionPolicyType;
use datadog_api_client::datadogV2::model::ExecutionPolicyWriteAttributes;

#[tokio::main]
async fn main() {
    let body = ExecutionPolicyCreateRequest::new(ExecutionPolicyCreateRequestData::new(
        ExecutionPolicyWriteAttributes::new(
            ExecutionPolicyActionPattern::new(
                vec!["com.datadoghq.script.*".to_string()],
                ExecutionPolicyIntegration::INTEGRATION_SCRIPT,
            ),
            ExecutionPolicyEffect::ALLOW,
            "Cassette Execution Policy exampleexecutionpolicy".to_string(),
        ),
        ExecutionPolicyType::EXECUTION_POLICY,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateExecutionPolicy", true);
    let api = ExecutionPolicyAPI::with_config(configuration);
    let resp = api.create_execution_policy(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
