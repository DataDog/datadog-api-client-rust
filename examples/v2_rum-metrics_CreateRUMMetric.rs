// Create a RUM-based metric returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_metrics::RUMMetricsAPI;
use datadog_api_client::datadogV2::model::RUMMetricCompute;
use datadog_api_client::datadogV2::model::RUMMetricComputeAggregationType;
use datadog_api_client::datadogV2::model::RUMMetricCreateAttributes;
use datadog_api_client::datadogV2::model::RUMMetricCreateData;
use datadog_api_client::datadogV2::model::RUMMetricCreateRequest;
use datadog_api_client::datadogV2::model::RUMMetricEventType;
use datadog_api_client::datadogV2::model::RUMMetricFilter;
use datadog_api_client::datadogV2::model::RUMMetricGroupBy;
use datadog_api_client::datadogV2::model::RUMMetricType;
use datadog_api_client::datadogV2::model::RUMMetricUniqueness;
use datadog_api_client::datadogV2::model::RUMMetricUniquenessWhen;

#[tokio::main]
async fn main() {
    let body = RUMMetricCreateRequest::new(RUMMetricCreateData::new(
        RUMMetricCreateAttributes::new(
            RUMMetricCompute::new(RUMMetricComputeAggregationType::DISTRIBUTION)
                .include_percentiles(true)
                .path("@duration".to_string()),
            RUMMetricEventType::SESSION,
        )
        .filter(RUMMetricFilter::new("@service:web-ui".to_string()))
        .group_by(vec![
            RUMMetricGroupBy::new("@browser.name".to_string()).tag_name("browser_name".to_string())
        ])
        .uniqueness(RUMMetricUniqueness::new(
            RUMMetricUniquenessWhen::WHEN_MATCH,
        )),
        "examplerummetric".to_string(),
        RUMMetricType::RUM_METRICS,
    ));
    let configuration = datadog::Configuration::new();
    let api = RUMMetricsAPI::with_config(configuration);
    let resp = api.create_rum_metric(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
