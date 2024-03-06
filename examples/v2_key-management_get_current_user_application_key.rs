// Get one application key owned by current user returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_key_management::*;

#[tokio::main]
async fn main() {
    let configuration = Configuration::new();
    let api = KeyManagementAPI::with_config(configuration);
    let resp = api
        .get_current_user_application_key("app_key_id".to_string())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}