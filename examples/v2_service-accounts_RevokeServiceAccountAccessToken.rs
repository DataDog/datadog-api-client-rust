// Revoke an access token for a service account returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_service_accounts::ServiceAccountsAPI;

#[tokio::main]
async fn main() {
    // there is a valid "service_account_user" in the system
    let service_account_user_data_id = std::env::var("SERVICE_ACCOUNT_USER_DATA_ID").unwrap();

    // there is a valid "service_account_access_token" for "service_account_user"
    let service_account_access_token_data_id =
        std::env::var("SERVICE_ACCOUNT_ACCESS_TOKEN_DATA_ID").unwrap();
    let configuration = datadog::Configuration::new();
    let api = ServiceAccountsAPI::with_config(configuration);
    let resp = api
        .revoke_service_account_access_token(
            service_account_user_data_id.clone(),
            service_account_access_token_data_id.clone(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
