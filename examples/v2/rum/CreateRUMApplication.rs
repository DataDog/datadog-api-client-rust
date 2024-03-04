// Create a new RUM application returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_rum::RUMAPI;
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
        RUMApplicationCreateRequest::new(
            RUMApplicationCreate::new(
                RUMApplicationCreateAttributes::new("test-rum-5c67ebb32077e1d9".to_string()).type_("ios".to_string()),
                RUMApplicationCreateType::RUM_APPLICATION_CREATE,
            ),
        );
    let configuration = Configuration::new();
    let api = RUMAPI::with_config(configuration);
    let resp = api.create_rum_application(body).await;
    if let Ok(Some(value)) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
