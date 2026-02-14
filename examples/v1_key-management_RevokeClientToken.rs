// Revoke client token returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV1::api_key_management::KeyManagementAPI;
use datadog_api_client::datadogV1::model::ClientTokenRevokeRequest;

#[tokio::main]
async fn main() {
    let body = ClientTokenRevokeRequest::new("1234567890abcdef1234567890abcdef123".to_string());
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v1.RevokeClientToken", true);
    let api = KeyManagementAPI::with_config(configuration);
    let resp = api.revoke_client_token(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
