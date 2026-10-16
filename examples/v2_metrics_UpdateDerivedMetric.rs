// Update a derived metric returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_metrics::MetricsAPI;
use datadog_api_client::datadogV2::model::DerivedMetricAttributes;
use datadog_api_client::datadogV2::model::DerivedMetricCreateData;
use datadog_api_client::datadogV2::model::DerivedMetricCreateRequest;
use datadog_api_client::datadogV2::model::DerivedMetricDefinition;
use datadog_api_client::datadogV2::model::DerivedMetricQuery;
use datadog_api_client::datadogV2::model::DerivedMetricsType;

#[tokio::main]
async fn main() {
    // there is a valid "derived_metric" in the system
    let derived_metric_data_id = std::env::var("DERIVED_METRIC_DATA_ID").unwrap();
    let body = DerivedMetricCreateRequest::new(DerivedMetricCreateData::new(
        DerivedMetricAttributes::new(DerivedMetricDefinition::new(
            "a * 2".to_string(),
            vec![DerivedMetricQuery::new(
                "metrics".to_string(),
                "a".to_string(),
                "avg:system.cpu.user{*}".to_string(),
            )],
        )),
        derived_metric_data_id.clone(),
        DerivedMetricsType::DERIVED_METRICS,
    ));
    let configuration = datadog::Configuration::new();
    let api = MetricsAPI::with_config(configuration);
    let resp = api
        .update_derived_metric(derived_metric_data_id.clone(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
