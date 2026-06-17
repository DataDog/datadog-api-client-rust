// Get a delegated token returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_delegated_token::DelegatedTokenAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = DelegatedTokenAPI::with_config(configuration);
    let resp = api.get_delegated_token().await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
