// Update pipeline order returns "OK" response
use chrono::prelude::{DateTime, Utc};
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_logs_pipelines::*;
use datadog_api_client::datadogV1::model::*;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let body = LogsPipelinesOrder::new(vec![
        "tags".to_string(),
        "org_ids".to_string(),
        "products".to_string(),
    ]);
    let configuration = Configuration::new();
    let api = LogsPipelinesAPI::with_config(configuration);
    let resp = api.update_logs_pipeline_order(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
