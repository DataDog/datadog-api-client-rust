// Upsert an OAuth2 client scopes restriction returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_o_auth2_client_public::OAuth2ClientPublicAPI;
use datadog_api_client::datadogV2::model::OAuthOidcScope;
use datadog_api_client::datadogV2::model::UpsertOAuthScopesRestrictionData;
use datadog_api_client::datadogV2::model::UpsertOAuthScopesRestrictionDataAttributes;
use datadog_api_client::datadogV2::model::UpsertOAuthScopesRestrictionRequest;
use datadog_api_client::datadogV2::model::UpsertOAuthScopesRestrictionType;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body = UpsertOAuthScopesRestrictionRequest::new(
        UpsertOAuthScopesRestrictionData::new(
            UpsertOAuthScopesRestrictionType::UPSERT_SCOPES_RESTRICTION,
        )
        .attributes(
            UpsertOAuthScopesRestrictionDataAttributes::new()
                .oidc_scopes(vec![OAuthOidcScope::OPENID, OAuthOidcScope::EMAIL])
                .permission_scopes(vec![
                    "dashboards_read".to_string(),
                    "metrics_read".to_string(),
                ]),
        ),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpsertScopesRestriction", true);
    let api = OAuth2ClientPublicAPI::with_config(configuration);
    let resp = api
        .upsert_scopes_restriction(
            Uuid::parse_str("fafa8e1c-36a5-11f0-a83d-da7ad0900001").expect("invalid UUID"),
            body,
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
