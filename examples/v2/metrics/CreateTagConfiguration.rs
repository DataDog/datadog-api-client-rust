// Create a tag configuration returns "Created" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_metrics::MetricsAPI;
use datadog_api_client::datadogV2::model::*;
use std::ops::Add;
use std::time::{
    Duration,
    SystemTime,
    UNIX_EPOCH,
};

#[tokio::main]
async fn main() {
    let body =
        MetricTagConfigurationCreateRequest::new(
            MetricTagConfigurationCreateData::new(
                "ExampleMetric".to_string(),
                MetricTagConfigurationType::MANAGE_TAGS,
            ).attributes(
                MetricTagConfigurationCreateAttributes::new(
                    MetricTagConfigurationMetricTypes::GAUGE,
                    vec!["app".to_string(), "datacenter".to_string()],
                ),
            ),
        );
    let configuration = Configuration::new();
    let api = MetricsAPI::with_config(configuration);
    let resp = api.create_tag_configuration(body).await;
    if let Ok(Some(value)) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
