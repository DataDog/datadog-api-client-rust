// Get dataset dependencies returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::SecurityMonitoringDatasetDependenciesAttributesRequest;
use datadog_api_client::datadogV2::model::SecurityMonitoringDatasetDependenciesDataRequest;
use datadog_api_client::datadogV2::model::SecurityMonitoringDatasetDependenciesRequest;
use datadog_api_client::datadogV2::model::SecurityMonitoringDatasetDependenciesType;

#[tokio::main]
async fn main() {
    let body = SecurityMonitoringDatasetDependenciesRequest::new(
        SecurityMonitoringDatasetDependenciesDataRequest::new(
            SecurityMonitoringDatasetDependenciesAttributesRequest::new(vec![
                "dataset-1".to_string()
            ]),
            SecurityMonitoringDatasetDependenciesType::SECURITY_MONITORING_DATASET_DEPENDENCIES,
        ),
    );
    let mut configuration = datadog::Configuration::new();
    configuration
        .set_unstable_operation_enabled("v2.BatchGetSecurityMonitoringDatasetDependencies", true);
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api
        .batch_get_security_monitoring_dataset_dependencies(body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
