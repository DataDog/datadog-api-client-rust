// Create a deployment returns "CREATED" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_fleet_automation::FleetAutomationAPI;
use datadog_api_client::datadogV2::model::FleetDeploymentConfigureAttributes;
use datadog_api_client::datadogV2::model::FleetDeploymentConfigureCreate;
use datadog_api_client::datadogV2::model::FleetDeploymentConfigureCreateRequest;
use datadog_api_client::datadogV2::model::FleetDeploymentFileOp;
use datadog_api_client::datadogV2::model::FleetDeploymentOperation;
use datadog_api_client::datadogV2::model::FleetDeploymentResourceType;
use serde_json::Value;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let body = FleetDeploymentConfigureCreateRequest::new(FleetDeploymentConfigureCreate::new(
        FleetDeploymentConfigureAttributes::new(vec![FleetDeploymentOperation::new(
            FleetDeploymentFileOp::MERGE_PATCH,
            "/datadog.yaml".to_string(),
        )
        .patch(BTreeMap::from([
            ("log_level".to_string(), Value::from("debug")),
            ("logs_enabled".to_string(), Value::from("True")),
        ]))])
        .filter_query("env:prod AND service:web".to_string()),
        FleetDeploymentResourceType::DEPLOYMENT,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateFleetDeploymentConfigure", true);
    let api = FleetAutomationAPI::with_config(configuration);
    let resp = api.create_fleet_deployment_configure(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
