// Trigger a deployment gate evaluation returns "Accepted" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_deployment_gates::DeploymentGatesAPI;
use datadog_api_client::datadogV2::model::DeploymentGatesEvaluationConfiguration;
use datadog_api_client::datadogV2::model::DeploymentGatesEvaluationRequest;
use datadog_api_client::datadogV2::model::DeploymentGatesEvaluationRequestAttributes;
use datadog_api_client::datadogV2::model::DeploymentGatesEvaluationRequestData;
use datadog_api_client::datadogV2::model::DeploymentGatesEvaluationRequestDataType;
use datadog_api_client::datadogV2::model::DeploymentGatesEvaluationRule;
use datadog_api_client::datadogV2::model::DeploymentGatesMonitorRule;
use datadog_api_client::datadogV2::model::DeploymentGatesMonitorRuleOptions;
use datadog_api_client::datadogV2::model::DeploymentGatesMonitorRuleType;

#[tokio::main]
async fn main() {
    let body = DeploymentGatesEvaluationRequest::new(DeploymentGatesEvaluationRequestData::new(
        DeploymentGatesEvaluationRequestAttributes::new(
            "staging".to_string(),
            "transaction-backend".to_string(),
        )
        .configuration(
            DeploymentGatesEvaluationConfiguration::new(vec![
                DeploymentGatesEvaluationRule::DeploymentGatesMonitorRule(Box::new(
                    DeploymentGatesMonitorRule::new(
                        "error rate monitors".to_string(),
                        DeploymentGatesMonitorRuleType::MONITOR,
                    )
                    .dry_run(false)
                    .options(
                        DeploymentGatesMonitorRuleOptions::new(
                            "service:transaction-backend env:production".to_string(),
                        )
                        .duration(300),
                    ),
                )),
            ])
            .dry_run(false),
        )
        .identifier("pre-deploy".to_string())
        .primary_tag("region:us-east-1".to_string())
        .version("v1.2.3".to_string()),
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
