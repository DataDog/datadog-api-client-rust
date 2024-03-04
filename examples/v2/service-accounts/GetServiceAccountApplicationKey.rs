// Get one application key for this service account returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_service_accounts::ServiceAccountsAPI;
use datadog_api_client::datadogV2::model::*;
use std::ops::Add;
use std::time::{
    Duration,
    SystemTime,
    UNIX_EPOCH,
};

#[tokio::main]
async fn main() {
    // there is a valid "service_account_user" in the system
    let service_account_user_data_id = std::env::var("SERVICE_ACCOUNT_USER_DATA_ID").unwrap();

    // there is a valid "service_account_application_key" for "service_account_user"
    let service_account_application_key_data_id = std::env::var("SERVICE_ACCOUNT_APPLICATION_KEY_DATA_ID").unwrap();
    let configuration = Configuration::new();
    let api = ServiceAccountsAPI::with_config(configuration);
    let resp = api.get_service_account_application_key().await;
    if let Ok(Some(value)) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
