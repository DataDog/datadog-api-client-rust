// Update Fastly account returns "OK" response
use chrono::prelude::*;
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_fastly_integration::*;
use datadog_api_client::datadogV2::model::*;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    // there is a valid "fastly_account" in the system
    let fastly_account_data_id = std::env::var("FASTLY_ACCOUNT_DATA_ID").unwrap();
    let body =
        FastlyAccountUpdateRequest::new(
            FastlyAccountUpdateRequestData::new()
                .attributes(FastlyAccountUpdateRequestAttributes::new().api_key("update-secret".to_string()))
                .type_(FastlyAccountType::FASTLY_ACCOUNTS),
        );
    let configuration = Configuration::new();
    let api = FastlyIntegrationAPI::with_config(configuration);
    let resp = api.update_fastly_account(fastly_account_data_id, body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
