// List execution policies with query parameters returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_execution_policy::ExecutionPolicyAPI;
use datadog_api_client::datadogV2::api_execution_policy::ListExecutionPoliciesOptionalParams;
use datadog_api_client::datadogV2::model::ExecutionPolicyEffect;
use datadog_api_client::datadogV2::model::ExecutionPolicyIntegration;

#[tokio::main]
async fn main() {
    // there is a valid "execution_policy" in the system
    let execution_policy_data_attributes_created_by =
        std::env::var("EXECUTION_POLICY_DATA_ATTRIBUTES_CREATED_BY").unwrap();
    let execution_policy_data_attributes_name =
        std::env::var("EXECUTION_POLICY_DATA_ATTRIBUTES_NAME").unwrap();
    let execution_policy_data_id = std::env::var("EXECUTION_POLICY_DATA_ID").unwrap();
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ListExecutionPolicies", true);
    let api = ExecutionPolicyAPI::with_config(configuration);
    let resp = api
        .list_execution_policies(
            ListExecutionPoliciesOptionalParams::default()
                .page_size(10)
                .page_number(0)
                .filter_name(execution_policy_data_attributes_name.clone())
                .filter_ids(vec![execution_policy_data_id.clone()])
                .filter_integration(vec![ExecutionPolicyIntegration::INTEGRATION_SCRIPT])
                .filter_effects(vec![ExecutionPolicyEffect::ALLOW])
                .filter_creator_ids(vec![execution_policy_data_attributes_created_by.clone()])
                .sort(vec!["-created_at".to_string()]),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
