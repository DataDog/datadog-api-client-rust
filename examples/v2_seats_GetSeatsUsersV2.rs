// Get seats users for a product code returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_seats::GetSeatsUsersV2OptionalParams;
use datadog_api_client::datadogV2::api_seats::SeatsAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = SeatsAPI::with_config(configuration);
    let resp = api
        .get_seats_users_v2(
            "product_code".to_string(),
            GetSeatsUsersV2OptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
