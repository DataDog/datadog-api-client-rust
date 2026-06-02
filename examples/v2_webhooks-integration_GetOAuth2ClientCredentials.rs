// Get an OAuth2 client credentials auth method returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_webhooks_integration::WebhooksIntegrationAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = WebhooksIntegrationAPI::with_config(configuration);
    let resp = api
        .get_o_auth2_client_credentials("auth_method_id".to_string())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
