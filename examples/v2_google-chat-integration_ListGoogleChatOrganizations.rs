// Get all Google Chat organization bindings returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_google_chat_integration::GoogleChatIntegrationAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = GoogleChatIntegrationAPI::with_config(configuration);
    let resp = api.list_google_chat_organizations().await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
