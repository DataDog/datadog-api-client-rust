// Get the schedule on-call user returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_on_call::GetScheduleOnCallUserOptionalParams;
use datadog_api_client::datadogV2::api_on_call::OnCallAPI;

#[tokio::main]
async fn main() {
    // there is a valid "schedule" in the system
    let schedule_data_id = std::env::var("SCHEDULE_DATA_ID").unwrap();
    let configuration = datadog::Configuration::new();
    let api = OnCallAPI::with_config(configuration);
    let resp = api
        .get_schedule_on_call_user(
            schedule_data_id.clone(),
            GetScheduleOnCallUserOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
