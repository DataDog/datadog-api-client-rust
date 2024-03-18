// Delete a custom variable returns "OK" response
use chrono::prelude::{DateTime, Utc};
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_webhooks_integration::*;
use datadog_api_client::datadogV1::model::*;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    // there is a valid "webhook_custom_variable" in the system
    let webhook_custom_variable_name = std::env::var("WEBHOOK_CUSTOM_VARIABLE_NAME").unwrap();
    let configuration = Configuration::new();
    let api = WebhooksIntegrationAPI::with_config(configuration);
    let resp = api
        .delete_webhooks_integration_custom_variable(webhook_custom_variable_name.clone())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
