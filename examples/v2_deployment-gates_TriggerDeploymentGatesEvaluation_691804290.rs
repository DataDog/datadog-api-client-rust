// Trigger a deployment gates evaluation returns "Accepted" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_deployment_gates::DeploymentGatesAPI;
use datadog_api_client::datadogV2::model::DeploymentGatesEvaluationRequest;
use datadog_api_client::datadogV2::model::DeploymentGatesEvaluationRequestAttributes;
use datadog_api_client::datadogV2::model::DeploymentGatesEvaluationRequestData;
use datadog_api_client::datadogV2::model::DeploymentGatesEvaluationRequestDataType;

#[tokio::main]
async fn main() {
    // there is a valid "deployment_gate" in the system
    let deployment_gate_data_attributes_identifier =
        std::env::var("DEPLOYMENT_GATE_DATA_ATTRIBUTES_IDENTIFIER").unwrap();
    let body = DeploymentGatesEvaluationRequest::new(DeploymentGatesEvaluationRequestData::new(
        DeploymentGatesEvaluationRequestAttributes::new(
            "production".to_string(),
            "my-service".to_string(),
        )
        .identifier(deployment_gate_data_attributes_identifier.clone()),
        DeploymentGatesEvaluationRequestDataType::DEPLOYMENT_GATES_EVALUATION_REQUEST,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.TriggerDeploymentGatesEvaluation", true);
    let api = DeploymentGatesAPI::with_config(configuration);
    let resp = api.trigger_deployment_gates_evaluation(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
