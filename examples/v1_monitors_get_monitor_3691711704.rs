// Get a synthetics monitor's details
use chrono::prelude::{DateTime, Utc};
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_monitors::*;
use datadog_api_client::datadogV1::model::*;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    // there is a valid "synthetics_api_test" in the system
    let synthetics_api_test_monitor_id: i64 = std::env::var("SYNTHETICS_API_TEST_MONITOR_ID")
        .unwrap()
        .parse()
        .unwrap();
    let configuration = Configuration::new();
    let api = MonitorsAPI::with_config(configuration);
    let resp = api
        .get_monitor(
            synthetics_api_test_monitor_id.clone(),
            GetMonitorOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
