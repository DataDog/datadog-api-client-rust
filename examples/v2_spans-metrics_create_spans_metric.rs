// Create a span-based metric returns "OK" response
use chrono::prelude::*;
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_spans_metrics::*;
use datadog_api_client::datadogV2::model::*;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let body =
        SpansMetricCreateRequest::new(
            SpansMetricCreateData::new(
                SpansMetricCreateAttributes::new(
                    SpansMetricCompute::new(SpansMetricComputeAggregationType::DISTRIBUTION)
                        .include_percentiles(false)
                        .path("@duration".to_string()),
                )
                    .filter(SpansMetricFilter::new().query("@http.status_code:200 service:my-service".to_string()))
                    .group_by(
                        vec![
                            SpansMetricGroupBy::new("resource_name".to_string()).tag_name("resource_name".to_string())
                        ],
                    ),
                "ExampleSpansMetric".to_string(),
                SpansMetricType::SPANS_METRICS,
            ),
        );
    let configuration = Configuration::new();
    let api = SpansMetricsAPI::with_config(configuration);
    let resp = api.create_spans_metric(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
