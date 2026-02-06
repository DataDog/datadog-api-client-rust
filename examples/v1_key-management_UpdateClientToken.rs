// Update client token returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV1::api_key_management::KeyManagementAPI;
use datadog_api_client::datadogV1::model::ClientTokenUpdateRequest;

#[tokio::main]
async fn main() {
    let body = ClientTokenUpdateRequest::new(
        "1234567890abcdef1234567890abcdef123".to_string(),
        "Updated Client Token Name".to_string(),
    )
    .origin_urls(vec!["https://example.com".to_string()]);
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v1.UpdateClientToken", true);
    let api = KeyManagementAPI::with_config(configuration);
    let resp = api.update_client_token(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
