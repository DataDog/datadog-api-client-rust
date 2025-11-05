// Trigger a schedule deployment returns "CREATED - Deployment successfully
// created and started." response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_fleet_automation::FleetAutomationAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.TriggerFleetSchedule", true);
    let api = FleetAutomationAPI::with_config(configuration);
    let resp = api.trigger_fleet_schedule("id".to_string()).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
