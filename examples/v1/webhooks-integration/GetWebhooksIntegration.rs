// Get a webhook integration returns "OK" response
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
    // there is a valid "webhook" in the system
    let webhook_name = std::env::var("WEBHOOK_NAME").unwrap();
    let configuration = Configuration::new();
    let api = WebhooksIntegrationAPI::with_config(configuration);
    let resp = api.get_webhooks_integration().await;
    if let Ok(Some(value)) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
