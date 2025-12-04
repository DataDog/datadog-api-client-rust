// Get detailed information about an agent returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_fleet_automation::FleetAutomationAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.GetFleetAgentInfo", true);
    let api = FleetAutomationAPI::with_config(configuration);
    let resp = api.get_fleet_agent_info("agent_key".to_string()).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
