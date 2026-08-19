// Get an execution policy returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_execution_policy::ExecutionPolicyAPI;

#[tokio::main]
async fn main() {
    // there is a valid "execution_policy" in the system
    let execution_policy_data_id = std::env::var("EXECUTION_POLICY_DATA_ID").unwrap();
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.GetExecutionPolicy", true);
    let api = ExecutionPolicyAPI::with_config(configuration);
    let resp = api
        .get_execution_policy(execution_policy_data_id.clone())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
