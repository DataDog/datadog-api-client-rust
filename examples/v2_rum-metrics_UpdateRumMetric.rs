// Update a rum-based metric returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_metrics::RumMetricsAPI;
use datadog_api_client::datadogV2::model::RumMetricFilter;
use datadog_api_client::datadogV2::model::RumMetricGroupBy;
use datadog_api_client::datadogV2::model::RumMetricType;
use datadog_api_client::datadogV2::model::RumMetricUpdateAttributes;
use datadog_api_client::datadogV2::model::RumMetricUpdateCompute;
use datadog_api_client::datadogV2::model::RumMetricUpdateData;
use datadog_api_client::datadogV2::model::RumMetricUpdateRequest;

#[tokio::main]
async fn main() {
    // there is a valid "rum_metric" in the system
    let rum_metric_data_id = std::env::var("RUM_METRIC_DATA_ID").unwrap();
    let body = RumMetricUpdateRequest::new(
        RumMetricUpdateData::new(
            RumMetricUpdateAttributes::new()
                .compute(RumMetricUpdateCompute::new().include_percentiles(false))
                .filter(RumMetricFilter::new("@service:rum-config".to_string()))
                .group_by(vec![RumMetricGroupBy::new("@browser.version".to_string())
                    .tag_name("browser_version".to_string())]),
            RumMetricType::RUM_METRICS,
        )
        .id(rum_metric_data_id.clone()),
    );

    let configuration = datadog::Configuration::new();
    let api = RumMetricsAPI::with_config(configuration);
    let resp = api
        .update_rum_metric(rum_metric_data_id.clone(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
