// Add Fastly account returns "CREATED" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_fastly_integration::FastlyIntegrationAPI;
use datadog_api_client::datadogV2::model::*;
use std::ops::Add;
use std::time::{
    Duration,
    SystemTime,
    UNIX_EPOCH,
};

#[tokio::main]
async fn main() {
    let body =
        FastlyAccountCreateRequest::new(
            FastlyAccountCreateRequestData::new(
                FastlyAccountCreateRequestAttributes::new(
                    "ExampleFastlyIntegration".to_string(),
                    "Example-Fastly-Integration".to_string(),
                ).services(vec![]),
                FastlyAccountType::FASTLY_ACCOUNTS,
            ),
        );
    let configuration = Configuration::new();
    let api = FastlyIntegrationAPI::with_config(configuration);
    let resp = api.create_fastly_account(body).await;
    if let Ok(Some(value)) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
