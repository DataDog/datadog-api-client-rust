// Create deployment gate returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_deployment_gates::DeploymentGatesAPI;
use datadog_api_client::datadogV2::model::CreateDeploymentGateParams;
use datadog_api_client::datadogV2::model::CreateDeploymentGateParamsData;
use datadog_api_client::datadogV2::model::CreateDeploymentGateParamsDataAttributes;
use datadog_api_client::datadogV2::model::DeploymentGateDataType;

#[tokio::main]
async fn main() {
    let body = CreateDeploymentGateParams::new(CreateDeploymentGateParamsData::new(
        CreateDeploymentGateParamsDataAttributes::new(
            "production".to_string(),
            "my-service".to_string(),
        )
        .dry_run(false)
        .identifier("my-gate-1".to_string()),
        DeploymentGateDataType::DEPLOYMENT_GATE,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateDeploymentGate", true);
    let api = DeploymentGatesAPI::with_config(configuration);
    let resp = api.create_deployment_gate(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
