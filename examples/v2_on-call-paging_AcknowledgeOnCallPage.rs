// Acknowledge On-Call Page returns "Accepted." response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_on_call_paging::OnCallPagingAPI;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = OnCallPagingAPI::with_config(configuration);
    let resp = api
        .acknowledge_on_call_page(
            Uuid::parse_str("15e74b8b-f865-48d0-bcc5-453323ed2c8f").expect("invalid UUID"),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
