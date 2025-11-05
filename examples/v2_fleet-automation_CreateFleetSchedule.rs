// Create a schedule returns "CREATED" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_fleet_automation::FleetAutomationAPI;
use datadog_api_client::datadogV2::model::FleetScheduleCreate;
use datadog_api_client::datadogV2::model::FleetScheduleCreateAttributes;
use datadog_api_client::datadogV2::model::FleetScheduleCreateRequest;
use datadog_api_client::datadogV2::model::FleetScheduleRecurrenceRule;
use datadog_api_client::datadogV2::model::FleetScheduleResourceType;
use datadog_api_client::datadogV2::model::FleetScheduleStatus;

#[tokio::main]
async fn main() {
    let body = FleetScheduleCreateRequest::new(FleetScheduleCreate::new(
        FleetScheduleCreateAttributes::new(
            "Weekly Production Agent Updates".to_string(),
            "env:prod AND service:web".to_string(),
            FleetScheduleRecurrenceRule::new(
                vec!["Mon".to_string(), "Wed".to_string(), "Fri".to_string()],
                1200,
                "02:00".to_string(),
                "America/New_York".to_string(),
            ),
        )
        .status(FleetScheduleStatus::ACTIVE)
        .version_to_latest(0),
        FleetScheduleResourceType::SCHEDULE,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateFleetSchedule", true);
    let api = FleetAutomationAPI::with_config(configuration);
    let resp = api.create_fleet_schedule(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
