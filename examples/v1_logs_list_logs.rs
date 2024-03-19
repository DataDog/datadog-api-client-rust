// Search logs returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_logs::LogsAPI;
use datadog_api_client::datadogV1::model::LogsListRequest;
use datadog_api_client::datadogV1::model::LogsListRequestTime;
use datadog_api_client::datadogV1::model::LogsSort;

#[tokio::main]
async fn main() {
    let body = LogsListRequest::new(LogsListRequestTime::new(
        "2020-02-02T02:02:02.202000+00:00".to_string(),
        "2020-02-20T02:02:02.202000+00:00".to_string(),
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
