// Search logs returns "OK" response
use chrono::prelude::{DateTime, Utc};
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_logs::*;
use datadog_api_client::datadogV1::model::*;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let body = LogsListRequest::new(LogsListRequestTime::new(
        DateTime::parse_from_rfc3339("2020-02-02T02:02:02.202000+00:00")
            .expect("Failed to parse datetime"),
        DateTime::parse_from_rfc3339("2020-02-20T02:02:02.202000+00:00")
            .expect("Failed to parse datetime"),
    ))
    .index("retention-3,retention-15".to_string())
    .query("service:web* AND @http.status_code:[200 TO 299]".to_string())
    .sort(LogsSort::TIME_ASCENDING);
    let configuration = Configuration::new();
    let api = LogsAPI::with_config(configuration);
    let resp = api.list_logs(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
