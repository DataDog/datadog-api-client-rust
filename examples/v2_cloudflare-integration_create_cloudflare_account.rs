// Add Cloudflare account returns "CREATED" response
use chrono::prelude::{DateTime, Utc};
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_cloudflare_integration::*;
use datadog_api_client::datadogV2::model::*;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let body = CloudflareAccountCreateRequest::new(CloudflareAccountCreateRequestData::new(
        CloudflareAccountCreateRequestAttributes::new(
            "fakekey".to_string(),
            "examplecloudflareintegration".to_string(),
        )
        .email("new@email".to_string()),
        CloudflareAccountType::CLOUDFLARE_ACCOUNTS,
    ));
    let configuration = Configuration::new();
    let api = CloudflareIntegrationAPI::with_config(configuration);
    let resp = api.create_cloudflare_account(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
