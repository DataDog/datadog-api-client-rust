// Search logs (POST) returns "OK" response with pagination
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_logs::ListLogsOptionalParams;
use datadog_api_client::datadogV2::api_logs::LogsAPI;
use datadog_api_client::datadogV2::model::LogsListRequest;
use datadog_api_client::datadogV2::model::LogsListRequestPage;
use datadog_api_client::datadogV2::model::LogsQueryFilter;
use datadog_api_client::datadogV2::model::LogsQueryOptions;
use datadog_api_client::datadogV2::model::LogsSort;
use datadog_api_client::datadogV2::model::LogsStorageTier;
use futures_util::pin_mut;
use futures_util::stream::StreamExt;

#[tokio::main]
async fn main() {
    let body =
        LogsListRequest::new()
            .filter(
                LogsQueryFilter::new()
                    .from("now-15m".to_string())
                    .indexes(vec!["main".to_string(), "web".to_string()])
                    .query("service:web* AND @http.status_code:[200 TO 299]".to_string())
                    .storage_tier(LogsStorageTier::INDEXES)
                    .to("now".to_string()),
            )
            .options(LogsQueryOptions::new().timezone("GMT".to_string()))
            .page(
                LogsListRequestPage::new()
                    .cursor(
                        "eyJzdGFydEF0IjoiQVFBQUFYS2tMS3pPbm40NGV3QUFBQUJCV0V0clRFdDZVbG8zY3pCRmNsbHJiVmxDWlEifQ==".to_string(),
                    )
                    .limit(25),
            )
            .sort(LogsSort::TIMESTAMP_ASCENDING);
    let configuration = datadog::Configuration::new();
    let api = LogsAPI::with_config(configuration);
    let response = api.list_logs_with_pagination(ListLogsOptionalParams::default().body(body));
    pin_mut!(response);
    while let Some(resp) = response.next().await {
        if let Ok(value) = resp {
            println!("{:#?}", value);
        } else {
            println!("{:#?}", resp.unwrap_err());
        }
    }
}
