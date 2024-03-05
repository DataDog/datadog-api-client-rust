// Submit a Service Check returns "Payload accepted" response
use chrono::prelude::*;
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_service_checks::*;
use datadog_api_client::datadogV1::model::*;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let body =
        vec![
            ServiceCheck::new(
                "app.ok".to_string(),
                "host".to_string(),
                ServiceCheckStatus::OK,
                vec!["test:ExampleServiceCheck".to_string()],
            )
        ];
    let configuration = Configuration::new();
    let api = ServiceChecksAPI::with_config(configuration);
    let resp = api.submit_service_check(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
