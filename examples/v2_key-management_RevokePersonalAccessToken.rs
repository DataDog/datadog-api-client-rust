// Revoke a personal access token returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_key_management::KeyManagementAPI;

#[tokio::main]
async fn main() {
    // there is a valid "personal_access_token" in the system
    let personal_access_token_data_id = std::env::var("PERSONAL_ACCESS_TOKEN_DATA_ID").unwrap();
    let configuration = datadog::Configuration::new();
    let api = KeyManagementAPI::with_config(configuration);
    let resp = api
        .revoke_personal_access_token(personal_access_token_data_id.clone())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
