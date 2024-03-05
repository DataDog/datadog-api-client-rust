// Search logs returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_logs::LogsAPI;
use datadog_api_client::datadogV1::model::*;

#[tokio::main]
async fn main() {
    let body =
        LogsListRequest::new(
            LogsListRequestTime::new(
                time.Date(2020, 2, 2, 2, 2, 2, 202000, time.UTC),
                time.Date(2020, 2, 20, 2, 2, 2, 202000, time.UTC),
            ),
        )
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
