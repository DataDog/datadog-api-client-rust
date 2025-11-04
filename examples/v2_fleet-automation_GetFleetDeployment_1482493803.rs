// Get a deployment by ID returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_fleet_automation::FleetAutomationAPI;
use datadog_api_client::datadogV2::api_fleet_automation::GetFleetDeploymentOptionalParams;

#[tokio::main]
async fn main() {
    // there is a valid "deployment" in the system
    let deployment_id = std::env::var("DEPLOYMENT_ID").unwrap();
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.GetFleetDeployment", true);
    let api = FleetAutomationAPI::with_config(configuration);
    let resp = api
        .get_fleet_deployment(
            deployment_id.clone(),
            GetFleetDeploymentOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
