// Create a rum-based metric returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_metrics::RumMetricsAPI;
use datadog_api_client::datadogV2::model::RumMetricCompute;
use datadog_api_client::datadogV2::model::RumMetricComputeAggregationType;
use datadog_api_client::datadogV2::model::RumMetricCreateAttributes;
use datadog_api_client::datadogV2::model::RumMetricCreateData;
use datadog_api_client::datadogV2::model::RumMetricCreateRequest;
use datadog_api_client::datadogV2::model::RumMetricEventType;
use datadog_api_client::datadogV2::model::RumMetricFilter;
use datadog_api_client::datadogV2::model::RumMetricGroupBy;
use datadog_api_client::datadogV2::model::RumMetricType;
use datadog_api_client::datadogV2::model::RumMetricUniqueness;
use datadog_api_client::datadogV2::model::RumMetricUniquenessWhen;

#[tokio::main]
async fn main() {
    let body = RumMetricCreateRequest::new(RumMetricCreateData::new(
        RumMetricCreateAttributes::new(
            RumMetricCompute::new(RumMetricComputeAggregationType::DISTRIBUTION)
                .include_percentiles(true)
                .path("@duration".to_string()),
            RumMetricEventType::SESSION,
        )
        .filter(RumMetricFilter::new("@service:web-ui".to_string()))
        .group_by(vec![
            RumMetricGroupBy::new("@browser.name".to_string()).tag_name("browser_name".to_string())
        ])
        .uniqueness(RumMetricUniqueness::new(
            RumMetricUniquenessWhen::WHEN_MATCH,
        )),
        "rum.sessions.webui.count".to_string(),
        RumMetricType::RUM_METRICS,
    ));

    let configuration = datadog::Configuration::new();
    let api = RumMetricsAPI::with_config(configuration);
    let resp = api.create_rum_metric(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
