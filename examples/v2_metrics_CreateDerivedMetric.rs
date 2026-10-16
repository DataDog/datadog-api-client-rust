// Create a derived metric returns "Created" response
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
    let body = DerivedMetricCreateRequest::new(DerivedMetricCreateData::new(
        DerivedMetricAttributes::new(DerivedMetricDefinition::new(
            "a".to_string(),
            vec![DerivedMetricQuery::new(
                "metrics".to_string(),
                "a".to_string(),
                "avg:system.cpu.user{*}".to_string(),
            )],
        )),
        "test.derived.examplemetric".to_string(),
        DerivedMetricsType::DERIVED_METRICS,
    ));
    let configuration = datadog::Configuration::new();
    let api = MetricsAPI::with_config(configuration);
    let resp = api.create_derived_metric(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
