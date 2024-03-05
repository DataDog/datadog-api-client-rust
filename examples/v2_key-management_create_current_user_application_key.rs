// Create an application key for current user returns "Created" response
use chrono::prelude::*;
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_key_management::*;
use datadog_api_client::datadogV2::model::*;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let body =
        ApplicationKeyCreateRequest::new(
            ApplicationKeyCreateData::new(
                ApplicationKeyCreateAttributes::new("Example-Key-Management".to_string()),
                ApplicationKeysType::APPLICATION_KEYS,
            ),
        );
    let configuration = Configuration::new();
    let api = KeyManagementAPI::with_config(configuration);
    let resp = api.create_current_user_application_key(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
