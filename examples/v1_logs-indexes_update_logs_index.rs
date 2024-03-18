// Update an index returns "OK" response
use chrono::prelude::{DateTime, Utc};
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_logs_indexes::*;
use datadog_api_client::datadogV1::model::*;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let body = LogsIndexUpdateRequest::new(LogsFilter::new().query("source:python".to_string()))
        .daily_limit(300000000)
        .disable_daily_limit(false)
        .exclusion_filters(vec![LogsExclusion::new("payment".to_string())
            .filter(LogsExclusionFilter::new(1.0).query("*".to_string()))])
        .num_retention_days(15);
    let configuration = Configuration::new();
    let api = LogsIndexesAPI::with_config(configuration);
    let resp = api.update_logs_index("name".to_string(), body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
