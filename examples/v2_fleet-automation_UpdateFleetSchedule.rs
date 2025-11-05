// Update a schedule returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_fleet_automation::FleetAutomationAPI;
use datadog_api_client::datadogV2::model::FleetSchedulePatch;
use datadog_api_client::datadogV2::model::FleetSchedulePatchAttributes;
use datadog_api_client::datadogV2::model::FleetSchedulePatchRequest;
use datadog_api_client::datadogV2::model::FleetScheduleRecurrenceRule;
use datadog_api_client::datadogV2::model::FleetScheduleResourceType;
use datadog_api_client::datadogV2::model::FleetScheduleStatus;

#[tokio::main]
async fn main() {
    let body = FleetSchedulePatchRequest::new(
        FleetSchedulePatch::new(FleetScheduleResourceType::SCHEDULE).attributes(
            FleetSchedulePatchAttributes::new()
                .name("Weekly Production Agent Updates".to_string())
                .query("env:prod AND service:web".to_string())
                .rule(FleetScheduleRecurrenceRule::new(
                    vec!["Mon".to_string(), "Wed".to_string(), "Fri".to_string()],
                    1200,
                    "02:00".to_string(),
                    "America/New_York".to_string(),
                ))
                .status(FleetScheduleStatus::ACTIVE)
                .version_to_latest(0),
        ),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdateFleetSchedule", true);
    let api = FleetAutomationAPI::with_config(configuration);
    let resp = api.update_fleet_schedule("id".to_string(), body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
