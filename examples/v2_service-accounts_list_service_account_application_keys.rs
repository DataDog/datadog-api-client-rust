// List application keys for this service account returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_service_accounts::*;

#[tokio::main]
async fn main() {
    // there is a valid "service_account_user" in the system
    let service_account_user_data_id = std::env::var("SERVICE_ACCOUNT_USER_DATA_ID").unwrap();
    let configuration = Configuration::new();
    let api = ServiceAccountsAPI::with_config(configuration);
    let resp = api
        .list_service_account_application_keys(
            service_account_user_data_id.clone(),
            ListServiceAccountApplicationKeysOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
