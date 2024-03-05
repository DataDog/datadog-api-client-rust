// Search test logs returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_logs::LogsAPI;
use datadog_api_client::datadogV1::model::*;
use std::ops::Add;
use std::time::{
    UNIX_EPOCH,
    Duration,
    SystemTime,
};

#[tokio::main]
async fn main() {
    let body =
        LogsListRequest::new(
            LogsListRequestTime::new(
                SystemTime::now().add(Duration::from_secs(-1 * 3600)).as_secs() as i64,
                SystemTime::now().as_secs() as i64,
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
