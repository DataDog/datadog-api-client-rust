// Search logs returns "OK" response with pagination
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_logs::*;
use datadog_api_client::datadogV2::model::*;

#[tokio::main]
async fn main() {
    let body =
        LogsListRequest::new()
            .filter(
                LogsQueryFilter::new()
                    .from("now-15m".to_string())
                    .indexes(vec!["main".to_string()])
                    .to("now".to_string()),
            )
            .options(LogsQueryOptions::new().timezone("GMT".to_string()))
            .page(LogsListRequestPage::new().limit(2))
            .sort(LogsSort::TIMESTAMP_ASCENDING);
    let configuration = Configuration::new();
    let api = LogsAPI::with_config(configuration);
    let resp = api.list_logs(ListLogsOptionalParams::default().body(body)).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
