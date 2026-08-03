// List all deployments returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_fleet_automation::FleetAutomationAPI;
use datadog_api_client::datadogV2::api_fleet_automation::ListFleetDeploymentsV2OptionalParams;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = FleetAutomationAPI::with_config(configuration);
    let resp = api
        .list_fleet_deployments_v2(ListFleetDeploymentsV2OptionalParams::default())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
