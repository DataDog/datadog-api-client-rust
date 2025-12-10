// Get rules for a deployment gate returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_deployment_gates::DeploymentGatesAPI;

#[tokio::main]
async fn main() {
    // there is a valid "deployment_gate" in the system
    let deployment_gate_data_id = std::env::var("DEPLOYMENT_GATE_DATA_ID").unwrap();
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.GetDeploymentGateRules", true);
    let api = DeploymentGatesAPI::with_config(configuration);
    let resp = api
        .get_deployment_gate_rules(deployment_gate_data_id.clone())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
