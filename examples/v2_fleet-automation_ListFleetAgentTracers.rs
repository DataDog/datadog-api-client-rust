// List tracers for a specific agent returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_fleet_automation::FleetAutomationAPI;
use datadog_api_client::datadogV2::api_fleet_automation::ListFleetAgentTracersOptionalParams;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ListFleetAgentTracers", true);
    let api = FleetAutomationAPI::with_config(configuration);
    let resp = api
        .list_fleet_agent_tracers(
            "agent_key".to_string(),
            ListFleetAgentTracersOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
