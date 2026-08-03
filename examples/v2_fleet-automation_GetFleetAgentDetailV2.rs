// Get detailed information about an agent returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_fleet_automation::FleetAutomationAPI;
use datadog_api_client::datadogV2::api_fleet_automation::GetFleetAgentDetailV2OptionalParams;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = FleetAutomationAPI::with_config(configuration);
    let resp = api
        .get_fleet_agent_detail_v2(
            "a1b2c3d4e5f67890a1b2c3d4e5f67890".to_string(),
            GetFleetAgentDetailV2OptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
