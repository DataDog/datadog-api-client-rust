// List deployment gate rule evaluations returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_deployment_gates::DeploymentGatesAPI;
use datadog_api_client::datadogV2::api_deployment_gates::ListDeploymentRuleEvaluationsOptionalParams;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = DeploymentGatesAPI::with_config(configuration);
    let resp = api
        .list_deployment_rule_evaluations(ListDeploymentRuleEvaluationsOptionalParams::default())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
