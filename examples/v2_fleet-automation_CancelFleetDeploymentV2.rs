// Cancel a deployment returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_fleet_automation::FleetAutomationAPI;

#[tokio::main]
async fn main() {
    // there is a valid "deployment" in the system
    let deployment_id = std::env::var("DEPLOYMENT_ID").unwrap();
    let configuration = datadog::Configuration::new();
    let api = FleetAutomationAPI::with_config(configuration);
    let resp = api.cancel_fleet_deployment_v2(deployment_id.clone()).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
