// Get a quick list of logs returns "OK" response
use chrono::prelude::*;
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_logs::*;
use datadog_api_client::datadogV2::model::*;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let configuration = Configuration::new();
    let api = LogsAPI::with_config(configuration);
    let resp =
        api
            .list_logs_get(
                ListLogsGetOptionalParams::default()
                    .filter_query("datadog-agent".to_string())
                    .filter_indexes(vec!["main".to_string()])
                    .filter_from(
                        Utc.with_ymd_and_hms(2020, 9, 17, 11, 48, 36).unwrap() + chrono::Duration::microseconds(0),
                    )
                    .filter_to(
                        Utc.with_ymd_and_hms(2020, 9, 17, 12, 48, 36).unwrap() + chrono::Duration::microseconds(0),
                    )
                    .page_limit(5),
            )
            .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
