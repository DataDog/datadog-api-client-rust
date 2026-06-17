// Get an intake API key returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_intake_key::IntakeKeyAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = IntakeKeyAPI::with_config(configuration);
    let resp = api.get_intake_key().await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
