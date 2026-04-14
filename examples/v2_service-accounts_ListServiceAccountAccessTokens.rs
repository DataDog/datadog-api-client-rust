// List access tokens for a service account returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_service_accounts::ListServiceAccountAccessTokensOptionalParams;
use datadog_api_client::datadogV2::api_service_accounts::ServiceAccountsAPI;

#[tokio::main]
async fn main() {
    // there is a valid "service_account_user" in the system
    let service_account_user_data_id = std::env::var("SERVICE_ACCOUNT_USER_DATA_ID").unwrap();
    let configuration = datadog::Configuration::new();
    let api = ServiceAccountsAPI::with_config(configuration);
    let resp = api
        .list_service_account_access_tokens(
            service_account_user_data_id.clone(),
            ListServiceAccountAccessTokensOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
