// List execution policies filtered by Slack returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_execution_policy::ExecutionPolicyAPI;
use datadog_api_client::datadogV2::api_execution_policy::ListExecutionPoliciesOptionalParams;
use datadog_api_client::datadogV2::model::ExecutionPolicyIntegration;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.list_execution_policies", true);
    let api = ExecutionPolicyAPI::with_config(configuration);
    let resp = api
        .list_execution_policies(
            ListExecutionPoliciesOptionalParams::default()
                .filter_integration(vec![ExecutionPolicyIntegration::INTEGRATION_SLACK]),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
