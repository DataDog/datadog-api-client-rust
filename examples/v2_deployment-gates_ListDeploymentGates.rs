// Get all deployment gates returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_deployment_gates::DeploymentGatesAPI;
use datadog_api_client::datadogV2::api_deployment_gates::ListDeploymentGatesOptionalParams;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ListDeploymentGates", true);
    let api = DeploymentGatesAPI::with_config(configuration);
    let resp = api
        .list_deployment_gates(ListDeploymentGatesOptionalParams::default())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
