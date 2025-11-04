// Upgrade hosts returns "CREATED" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_fleet_automation::FleetAutomationAPI;
use datadog_api_client::datadogV2::model::FleetDeploymentPackage;
use datadog_api_client::datadogV2::model::FleetDeploymentPackageUpgradeAttributes;
use datadog_api_client::datadogV2::model::FleetDeploymentPackageUpgradeCreate;
use datadog_api_client::datadogV2::model::FleetDeploymentPackageUpgradeCreateRequest;
use datadog_api_client::datadogV2::model::FleetDeploymentResourceType;

#[tokio::main]
async fn main() {
    let body =
        FleetDeploymentPackageUpgradeCreateRequest::new(FleetDeploymentPackageUpgradeCreate::new(
            FleetDeploymentPackageUpgradeAttributes::new(vec![FleetDeploymentPackage::new(
                "datadog-agent".to_string(),
                "7.52.0".to_string(),
            )])
            .filter_query("env:prod AND service:web".to_string()),
            FleetDeploymentResourceType::DEPLOYMENT,
        ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateFleetDeploymentUpgrade", true);
    let api = FleetAutomationAPI::with_config(configuration);
    let resp = api.create_fleet_deployment_upgrade(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
