// Get an OAuth2 client scopes restriction returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_o_auth2_client_public::OAuth2ClientPublicAPI;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.GetScopesRestriction", true);
    let api = OAuth2ClientPublicAPI::with_config(configuration);
    let resp = api
        .get_scopes_restriction(
            Uuid::parse_str("fafa8e1c-36a5-11f0-a83d-da7ad0900001").expect("invalid UUID"),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
