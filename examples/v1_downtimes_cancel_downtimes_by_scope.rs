// Cancel downtimes by scope returns "OK" response
use chrono::prelude::{DateTime, Utc};
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_downtimes::*;
use datadog_api_client::datadogV1::model::*;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    // there is a valid "downtime" in the system
    let downtime_scope_0 = std::env::var("DOWNTIME_SCOPE_0").unwrap();
    let body = CancelDowntimesByScopeRequest::new(downtime_scope_0.clone());
    let configuration = Configuration::new();
    let api = DowntimesAPI::with_config(configuration);
    let resp = api.cancel_downtimes_by_scope(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
