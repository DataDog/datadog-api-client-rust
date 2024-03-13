// Update a log-based metric returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_logs_metrics::*;
use datadog_api_client::datadogV2::model::*;

#[tokio::main]
async fn main() {
    // there is a valid "logs_metric" in the system
    let logs_metric_data_id = std::env::var("LOGS_METRIC_DATA_ID").unwrap();
    let body = LogsMetricUpdateRequest::new(LogsMetricUpdateData::new(
        LogsMetricUpdateAttributes::new().filter(
            LogsMetricFilter::new()
                .query("service:web* AND @http.status_code:[200 TO 299]-updated".to_string()),
        ),
        LogsMetricType::LOGS_METRICS,
    ));
    let configuration = Configuration::new();
    let api = LogsMetricsAPI::with_config(configuration);
    let resp = api
        .update_logs_metric(logs_metric_data_id.clone(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}