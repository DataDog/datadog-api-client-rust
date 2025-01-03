// Search logs (GET) returns "OK" response with pagination
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_logs::ListLogsGetOptionalParams;
use datadog_api_client::datadogV2::api_logs::LogsAPI;
use futures_util::pin_mut;
use futures_util::stream::StreamExt;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = LogsAPI::with_config(configuration);
    let response = api.list_logs_get_with_pagination(ListLogsGetOptionalParams::default());
    pin_mut!(response);
    while let Some(resp) = response.next().await {
        if let Ok(value) = resp {
            println!("{:#?}", value);
        } else {
            println!("{:#?}", resp.unwrap_err());
        }
    }
}
