// Delete an application key owned by current user returns "No Content" response
use chrono::prelude::{DateTime, Utc};
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_key_management::*;
use datadog_api_client::datadogV2::model::*;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    // there is a valid "application_key" in the system
    let application_key_data_id = std::env::var("APPLICATION_KEY_DATA_ID").unwrap();
    let configuration = Configuration::new();
    let api = KeyManagementAPI::with_config(configuration);
    let resp = api
        .delete_current_user_application_key(application_key_data_id.clone())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
