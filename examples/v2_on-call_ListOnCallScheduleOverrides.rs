// Get a list of all overrides for a schedule returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_on_call::ListOnCallScheduleOverridesOptionalParams;
use datadog_api_client::datadogV2::api_on_call::OnCallAPI;

#[tokio::main]
async fn main() {
    // there is a valid "schedule" in the system
    let schedule_data_id = std::env::var("SCHEDULE_DATA_ID").unwrap();
    let configuration = datadog::Configuration::new();
    let api = OnCallAPI::with_config(configuration);
    let resp = api
        .list_on_call_schedule_overrides(
            schedule_data_id.clone(),
            "2021-11-11T10:11:11+00:00".to_string(),
            "2021-11-11T12:11:11+00:00".to_string(),
            ListOnCallScheduleOverridesOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
