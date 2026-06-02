// Update an OAuth2 client credentials auth method returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_webhooks_integration::WebhooksIntegrationAPI;
use datadog_api_client::datadogV2::model::WebhooksOAuth2ClientCredentialsType;
use datadog_api_client::datadogV2::model::WebhooksOAuth2ClientCredentialsUpdateAttributes;
use datadog_api_client::datadogV2::model::WebhooksOAuth2ClientCredentialsUpdateData;
use datadog_api_client::datadogV2::model::WebhooksOAuth2ClientCredentialsUpdateRequest;

#[tokio::main]
async fn main() {
    let body = WebhooksOAuth2ClientCredentialsUpdateRequest::new(
        WebhooksOAuth2ClientCredentialsUpdateData::new(
            WebhooksOAuth2ClientCredentialsUpdateAttributes::new()
                .access_token_url("https://example.com/oauth/token".to_string())
                .audience(Some("https://api.example.com".to_string()))
                .client_id("my-client-id".to_string())
                .client_secret("my-client-secret".to_string())
                .name("my-oauth2-auth".to_string())
                .scope(Some("read:webhooks write:webhooks".to_string())),
            WebhooksOAuth2ClientCredentialsType::WEBHOOKS_AUTH_METHOD_OAUTH2_CLIENT_CREDENTIALS,
        ),
    );
    let configuration = datadog::Configuration::new();
    let api = WebhooksIntegrationAPI::with_config(configuration);
    let resp = api
        .update_o_auth2_client_credentials("auth_method_id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
