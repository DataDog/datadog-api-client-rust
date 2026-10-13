// List deployment gate evaluations returns "OK" response with pagination
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_deployment_gates::DeploymentGatesAPI;
use datadog_api_client::datadogV2::api_deployment_gates::ListDeploymentGateEvaluationsOptionalParams;
use futures_util::pin_mut;
use futures_util::stream::StreamExt;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.list_deployment_gate_evaluations", true);
    let api = DeploymentGatesAPI::with_config(configuration);
    let response = api.list_deployment_gate_evaluations_with_pagination(
        ListDeploymentGateEvaluationsOptionalParams::default(),
    );
    pin_mut!(response);
    while let Some(resp) = response.next().await {
        if let Ok(value) = resp {
            println!("{:#?}", value);
        } else {
            println!("{:#?}", resp.unwrap_err());
        }
    }
}
