// Update a custom variable returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_webhooks_integration::WebhooksIntegrationAPI;
use datadog_api_client::datadogV1::model::*;
use std::ops::Add;
use std::time::{
    Duration,
    SystemTime,
    UNIX_EPOCH,
};

#[tokio::main]
async fn main() {
    // there is a valid "webhook_custom_variable" in the system
    let webhook_custom_variable_name = std::env::var("WEBHOOK_CUSTOM_VARIABLE_NAME").unwrap();
    let body = WebhooksIntegrationCustomVariableUpdateRequest::new().value("variable-updated".to_string());
    let configuration = Configuration::new();
    let api = WebhooksIntegrationAPI::with_config(configuration);
    let resp = api.update_webhooks_integration_custom_variable(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
