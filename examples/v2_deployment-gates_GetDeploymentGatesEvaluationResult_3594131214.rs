// Get a deployment gates evaluation result returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_deployment_gates::DeploymentGatesAPI;

#[tokio::main]
async fn main() {
    // there is a valid "deployment_gates_evaluation" in the system
    let deployment_gates_evaluation_data_id =
        uuid::Uuid::parse_str(&std::env::var("DEPLOYMENT_GATES_EVALUATION_DATA_ID").unwrap())
            .expect("Invalid UUID");
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.GetDeploymentGatesEvaluationResult", true);
    let api = DeploymentGatesAPI::with_config(configuration);
    let resp = api
        .get_deployment_gates_evaluation_result(deployment_gates_evaluation_data_id.clone())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
