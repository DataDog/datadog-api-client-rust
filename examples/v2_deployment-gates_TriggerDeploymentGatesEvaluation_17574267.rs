// Trigger a deployment gates evaluation with an inline bits deployment analysis
// rule returns "Accepted" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_deployment_gates::DeploymentGatesAPI;
use datadog_api_client::datadogV2::model::DeploymentGatesBitsDeploymentAnalysisRule;
use datadog_api_client::datadogV2::model::DeploymentGatesBitsDeploymentAnalysisRuleOptions;
use datadog_api_client::datadogV2::model::DeploymentGatesBitsDeploymentAnalysisRuleType;
use datadog_api_client::datadogV2::model::DeploymentGatesEvaluationConfiguration;
use datadog_api_client::datadogV2::model::DeploymentGatesEvaluationRequest;
use datadog_api_client::datadogV2::model::DeploymentGatesEvaluationRequestAttributes;
use datadog_api_client::datadogV2::model::DeploymentGatesEvaluationRequestData;
use datadog_api_client::datadogV2::model::DeploymentGatesEvaluationRequestDataType;
use datadog_api_client::datadogV2::model::DeploymentGatesEvaluationRule;

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
        .configuration(
            DeploymentGatesEvaluationConfiguration::new(vec![
                DeploymentGatesEvaluationRule::DeploymentGatesBitsDeploymentAnalysisRule(Box::new(
                    DeploymentGatesBitsDeploymentAnalysisRule::new(
                        "bits deployment analysis".to_string(),
                        DeploymentGatesBitsDeploymentAnalysisRuleType::BITS_DEPLOYMENT_ANALYSIS,
                    )
                    .dry_run(false)
                    .options(DeploymentGatesBitsDeploymentAnalysisRuleOptions::new().duration(900)),
                )),
            ])
            .dry_run(false),
        )
        .identifier(deployment_gate_data_attributes_identifier.clone()),
        DeploymentGatesEvaluationRequestDataType::DEPLOYMENT_GATES_EVALUATION_REQUEST,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.trigger_deployment_gates_evaluation", true);
    let api = DeploymentGatesAPI::with_config(configuration);
    let resp = api.trigger_deployment_gates_evaluation(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
