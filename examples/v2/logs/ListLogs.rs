// Search logs returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_logs::LogsAPI;
use datadog_api_client::datadogV2::model::*;
use std::ops::Add;
use std::time::{
    Duration,
    SystemTime,
    UNIX_EPOCH,
};

#[tokio::main]
async fn main() {
    let body =
        LogsListRequest::new()
            .filter(
                LogsQueryFilter::new()
                    .from("2020-09-17T11:48:36+01:00".to_string())
                    .indexes(vec!["main".to_string()])
                    .query("datadog-agent".to_string())
                    .to("2020-09-17T12:48:36+01:00".to_string()),
            )
            .page(LogsListRequestPage::new().limit(5))
            .sort(LogsSort::TIMESTAMP_ASCENDING);
    let configuration = Configuration::new();
    let api = LogsAPI::with_config(configuration);
    let resp = api.list_logs(body).await;
    if let Ok(Some(value)) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
