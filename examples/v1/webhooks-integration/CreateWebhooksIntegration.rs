// Create a webhooks integration returns "OK" response
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
    let body =
        WebhooksIntegration::new(
            "Example-Webhooks-Integration".to_string(),
            "https://example.com/webhook".to_string(),
        );
    let configuration = Configuration::new();
    let api = WebhooksIntegrationAPI::with_config(configuration);
    let resp = api.create_webhooks_integration(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
