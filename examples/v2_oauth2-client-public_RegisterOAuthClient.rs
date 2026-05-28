// Register an OAuth2 client returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_o_auth2_client_public::OAuth2ClientPublicAPI;
use datadog_api_client::datadogV2::model::OAuthClientRegistrationGrantType;
use datadog_api_client::datadogV2::model::OAuthClientRegistrationRequest;
use datadog_api_client::datadogV2::model::OAuthClientRegistrationResponseType;

#[tokio::main]
async fn main() {
    let body = OAuthClientRegistrationRequest::new(
        "Example MCP Client".to_string(),
        vec!["https://example.com/oauth/callback".to_string()],
    )
    .client_uri("https://example.com".to_string())
    .grant_types(vec![
        OAuthClientRegistrationGrantType::AUTHORIZATION_CODE,
        OAuthClientRegistrationGrantType::REFRESH_TOKEN,
    ])
    .jwks_uri("https://example.com/.well-known/jwks.json".to_string())
    .logo_uri("https://example.com/logo.png".to_string())
    .policy_uri("https://example.com/privacy".to_string())
    .response_types(vec![OAuthClientRegistrationResponseType::CODE])
    .scope("openid profile".to_string())
    .token_endpoint_auth_method("none".to_string())
    .tos_uri("https://example.com/tos".to_string());
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.RegisterOAuthClient", true);
    let api = OAuth2ClientPublicAPI::with_config(configuration);
    let resp = api.register_o_auth_client(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
