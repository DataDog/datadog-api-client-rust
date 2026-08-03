// Upgrade hosts returns "CREATED" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_fleet_automation::FleetAutomationAPI;
use datadog_api_client::datadogV2::model::FleetDeploymentPackage;
use datadog_api_client::datadogV2::model::FleetDeploymentPackageUpgradeV2Attributes;
use datadog_api_client::datadogV2::model::FleetDeploymentPackageUpgradeV2Create;
use datadog_api_client::datadogV2::model::FleetDeploymentPackageUpgradeV2CreateRequest;
use datadog_api_client::datadogV2::model::FleetDeploymentResourceType;

#[tokio::main]
async fn main() {
    let body = FleetDeploymentPackageUpgradeV2CreateRequest::new(
        FleetDeploymentPackageUpgradeV2Create::new(
            FleetDeploymentPackageUpgradeV2Attributes::new(
                "env:prod AND service:example-fleet-automation".to_string(),
                vec![FleetDeploymentPackage::new(
                    "datadog-agent".to_string(),
                    "7.52.0".to_string(),
                )],
            ),
            FleetDeploymentResourceType::DEPLOYMENT,
        ),
    );
    let configuration = datadog::Configuration::new();
    let api = FleetAutomationAPI::with_config(configuration);
    let resp = api.create_fleet_deployment_upgrade_v2(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
