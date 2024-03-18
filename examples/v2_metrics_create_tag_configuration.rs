// Create a tag configuration returns "Created" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_metrics::*;
use datadog_api_client::datadogV2::model::*;

#[tokio::main]
async fn main() {
    let body = MetricTagConfigurationCreateRequest::new(
        MetricTagConfigurationCreateData::new(
            "ExampleMetric".to_string(),
            MetricTagConfigurationType::MANAGE_TAGS,
        )
        .attributes(MetricTagConfigurationCreateAttributes::new(
            MetricTagConfigurationMetricTypes::GAUGE,
            vec!["app".to_string(), "datacenter".to_string()],
        )),
    );
    let configuration = Configuration::new();
    let api = MetricsAPI::with_config(configuration);
    let resp = api
        .create_tag_configuration("ExampleMetric".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
