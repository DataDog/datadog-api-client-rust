// Get mobile hourly usage for RUM Sessions returns "OK" response
use chrono::prelude::{DateTime, Utc};
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_usage_metering::*;
use datadog_api_client::datadogV1::model::*;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let configuration = Configuration::new();
    let api = UsageMeteringAPI::with_config(configuration);
    let resp = api
        .get_usage_rum_sessions(
            DateTime::parse_from_rfc3339("2021-11-06T11:11:11+00:00")
                .expect("Failed to parse datetime"),
            GetUsageRumSessionsOptionalParams::default()
                .end_hr(
                    DateTime::parse_from_rfc3339("2021-11-08T11:11:11+00:00")
                        .expect("Failed to parse datetime"),
                )
                .type_("mobile".to_string()),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
