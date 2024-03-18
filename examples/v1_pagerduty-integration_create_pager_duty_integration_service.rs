// Create a new service object returns "OK" response
use chrono::prelude::{DateTime, Utc};
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_pager_duty_integration::*;
use datadog_api_client::datadogV1::model::*;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let body = PagerDutyService::new("".to_string(), "".to_string());
    let configuration = Configuration::new();
    let api = PagerDutyIntegrationAPI::with_config(configuration);
    let resp = api.create_pager_duty_integration_service(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
