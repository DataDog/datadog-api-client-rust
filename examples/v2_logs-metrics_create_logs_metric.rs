// Create a log-based metric returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_logs_metrics::*;
use datadog_api_client::datadogV2::model::*;

#[tokio::main]
async fn main() {
    let body = LogsMetricCreateRequest::new(LogsMetricCreateData::new(
        LogsMetricCreateAttributes::new(
            LogsMetricCompute::new(LogsMetricComputeAggregationType::DISTRIBUTION)
                .include_percentiles(true)
                .path("@duration".to_string()),
        ),
        "ExampleLogsMetric".to_string(),
        LogsMetricType::LOGS_METRICS,
    ));
    let configuration = Configuration::new();
    let api = LogsMetricsAPI::with_config(configuration);
    let resp = api.create_logs_metric(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
