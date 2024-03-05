// Search test logs returns "OK" response
use chrono::prelude::*;
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_logs::*;
use datadog_api_client::datadogV1::model::*;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let body =
        LogsListRequest::new(
            LogsListRequestTime::new(
                (Utc::now() + chrono::Duration::hours(-1)).to_rfc3339(),
                (Utc::now()).to_rfc3339(),
            ).timezone("Europe/Paris".to_string()),
        )
            .index("main".to_string())
            .query("host:Test*".to_string())
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
