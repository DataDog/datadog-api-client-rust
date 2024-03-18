// Update Cloudflare account returns "OK" response
use chrono::prelude::{DateTime, Utc};
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_cloudflare_integration::*;
use datadog_api_client::datadogV2::model::*;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    // there is a valid "cloudflare_account" in the system
    let cloudflare_account_data_id = std::env::var("CLOUDFLARE_ACCOUNT_DATA_ID").unwrap();
    let body = CloudflareAccountUpdateRequest::new(
        CloudflareAccountUpdateRequestData::new()
            .attributes(
                CloudflareAccountUpdateRequestAttributes::new("fakekey".to_string())
                    .email("new@email".to_string()),
            )
            .type_(CloudflareAccountType::CLOUDFLARE_ACCOUNTS),
    );
    let configuration = Configuration::new();
    let api = CloudflareIntegrationAPI::with_config(configuration);
    let resp = api
        .update_cloudflare_account(cloudflare_account_data_id.clone(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
