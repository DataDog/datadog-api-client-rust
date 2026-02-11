// Get users with seats returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_seats::GetSeatsUsersOptionalParams;
use datadog_api_client::datadogV2::api_seats::SeatsAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = SeatsAPI::with_config(configuration);
    let resp = api
        .get_seats_users(
            "incident_response".to_string(),
            GetSeatsUsersOptionalParams::default().page_limit(100),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
