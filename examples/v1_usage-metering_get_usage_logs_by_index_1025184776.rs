// Get hourly usage for Logs by Index returns "OK" response
use chrono::prelude::*;
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_usage_metering::*;
use datadog_api_client::datadogV1::model::*;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let configuration = Configuration::new();
    let api = UsageMeteringAPI::with_config(configuration);
    let resp =
        api
            .get_usage_logs_by_index(
                (Utc::now() + chrono::Duration::days(-5)).to_rfc3339(),
                GetUsageLogsByIndexOptionalParams
                ::default().end_hr((Utc::now() + chrono::Duration::days(-3)).to_rfc3339()),
            )
            .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
