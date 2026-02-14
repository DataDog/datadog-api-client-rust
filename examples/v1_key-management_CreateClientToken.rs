// Create client token returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV1::api_key_management::KeyManagementAPI;
use datadog_api_client::datadogV1::model::ClientTokenCreateRequest;

#[tokio::main]
async fn main() {
    let body = ClientTokenCreateRequest::new(
        "Example Client Token".to_string(),
        vec!["https://example.com".to_string()],
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v1.CreateClientToken", true);
    let api = KeyManagementAPI::with_config(configuration);
    let resp = api.create_client_token(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
