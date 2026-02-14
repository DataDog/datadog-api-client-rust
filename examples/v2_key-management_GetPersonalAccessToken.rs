// Get personal access token returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_key_management::KeyManagementAPI;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.GetPersonalAccessToken", true);
    let api = KeyManagementAPI::with_config(configuration);
    let resp = api
        .get_personal_access_token(
            Uuid::parse_str("00000000-0000-0000-0000-000000000000").expect("invalid UUID"),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
