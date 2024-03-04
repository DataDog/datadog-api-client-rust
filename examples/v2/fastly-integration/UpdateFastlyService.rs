// Update Fastly service returns "OK" response
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
        FastlyServiceRequest::new(
            FastlyServiceData::new(
                "abc123".to_string(),
                FastlyServiceType::FASTLY_SERVICES,
            ).attributes(
                FastlyServiceAttributes::new().tags(vec!["myTag".to_string(), "myTag2:myValue".to_string()]),
            ),
        );
    let configuration = Configuration::new();
    let api = FastlyIntegrationAPI::with_config(configuration);
    let resp = api.update_fastly_service(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
