// Update a tag configuration returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_metrics::*;
use datadog_api_client::datadogV2::model::*;

#[tokio::main]
async fn main() {
    // there is a valid "metric_tag_configuration" in the system
    let metric_tag_configuration_data_id =
        std::env::var("METRIC_TAG_CONFIGURATION_DATA_ID").unwrap();
    let body = MetricTagConfigurationUpdateRequest::new(
        MetricTagConfigurationUpdateData::new(
            metric_tag_configuration_data_id.clone(),
            MetricTagConfigurationType::MANAGE_TAGS,
        )
        .attributes(MetricTagConfigurationUpdateAttributes::new().tags(vec!["app".to_string()])),
    );
    let configuration = Configuration::new();
    let api = MetricsAPI::with_config(configuration);
    let resp = api
        .update_tag_configuration(metric_tag_configuration_data_id.clone(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
