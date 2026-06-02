// Create an OAuth2 client credentials auth method returns "CREATED" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_webhooks_integration::WebhooksIntegrationAPI;
use datadog_api_client::datadogV2::model::WebhooksOAuth2ClientCredentialsCreateAttributes;
use datadog_api_client::datadogV2::model::WebhooksOAuth2ClientCredentialsCreateData;
use datadog_api_client::datadogV2::model::WebhooksOAuth2ClientCredentialsCreateRequest;
use datadog_api_client::datadogV2::model::WebhooksOAuth2ClientCredentialsType;

#[tokio::main]
async fn main() {
    let body = WebhooksOAuth2ClientCredentialsCreateRequest::new(
        WebhooksOAuth2ClientCredentialsCreateData::new(
            WebhooksOAuth2ClientCredentialsCreateAttributes::new(
                "https://example.com/oauth/token".to_string(),
                "my-client-id".to_string(),
                "my-client-secret".to_string(),
                "my-oauth2-auth".to_string(),
            )
            .audience(Some("https://api.example.com".to_string()))
            .scope(Some("read:webhooks write:webhooks".to_string())),
            WebhooksOAuth2ClientCredentialsType::WEBHOOKS_AUTH_METHOD_OAUTH2_CLIENT_CREDENTIALS,
        ),
    );
    let configuration = datadog::Configuration::new();
    let api = WebhooksIntegrationAPI::with_config(configuration);
    let resp = api.create_o_auth2_client_credentials(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
