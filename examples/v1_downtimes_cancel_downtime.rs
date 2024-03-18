// Cancel a downtime returns "OK" response
use chrono::prelude::{DateTime, Utc};
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_downtimes::*;
use datadog_api_client::datadogV1::model::*;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    // there is a valid "downtime" in the system
    let downtime_id: i64 = std::env::var("DOWNTIME_ID").unwrap().parse().unwrap();
    let configuration = Configuration::new();
    let api = DowntimesAPI::with_config(configuration);
    let resp = api.cancel_downtime(downtime_id.clone()).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
