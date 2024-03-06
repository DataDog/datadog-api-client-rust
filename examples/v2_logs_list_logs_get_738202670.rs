// Get a list of logs returns "OK" response with pagination
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_logs::*;

#[tokio::main]
async fn main() {
    let configuration = Configuration::new();
    let api = LogsAPI::with_config(configuration);
    let resp = api
        .list_logs_get(ListLogsGetOptionalParams::default().page_limit(2))
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
