// Update a RUM-based metric returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_metrics::RUMMetricsAPI;
use datadog_api_client::datadogV2::model::RUMMetricFilter;
use datadog_api_client::datadogV2::model::RUMMetricGroupBy;
use datadog_api_client::datadogV2::model::RUMMetricType;
use datadog_api_client::datadogV2::model::RUMMetricUpdateAttributes;
use datadog_api_client::datadogV2::model::RUMMetricUpdateCompute;
use datadog_api_client::datadogV2::model::RUMMetricUpdateData;
use datadog_api_client::datadogV2::model::RUMMetricUpdateRequest;

#[tokio::main]
async fn main() {
    // there is a valid "rum_metric" in the system
    let rum_metric_data_id = std::env::var("RUM_METRIC_DATA_ID").unwrap();
    let body = RUMMetricUpdateRequest::new(
        RUMMetricUpdateData::new(
            RUMMetricUpdateAttributes::new()
                .compute(RUMMetricUpdateCompute::new().include_percentiles(false))
                .filter(RUMMetricFilter::new("@service:rum-config".to_string()))
                .group_by(vec![RUMMetricGroupBy::new("@browser.version".to_string())
                    .tag_name("browser_version".to_string())]),
            RUMMetricType::RUM_METRICS,
        )
        .id(rum_metric_data_id.clone()),
    );
    let configuration = datadog::Configuration::new();
    let api = RUMMetricsAPI::with_config(configuration);
    let resp = api
        .update_rum_metric(rum_metric_data_id.clone(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
