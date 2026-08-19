// Update an execution policy returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_execution_policy::ExecutionPolicyAPI;
use datadog_api_client::datadogV2::model::ExecutionPolicyActionPattern;
use datadog_api_client::datadogV2::model::ExecutionPolicyEffect;
use datadog_api_client::datadogV2::model::ExecutionPolicyIntegration;
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
                vec!["com.datadoghq.script.*".to_string()],
                ExecutionPolicyIntegration::INTEGRATION_SCRIPT,
            ),
            ExecutionPolicyEffect::ALLOW,
            "Cassette Execution Policy Updated".to_string(),
        ),
        execution_policy_data_id.clone(),
        ExecutionPolicyType::EXECUTION_POLICY,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdateExecutionPolicy", true);
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
