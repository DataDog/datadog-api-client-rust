// Create a dataset returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::SecurityMonitoringDatasetCreateAttributesRequest;
use datadog_api_client::datadogV2::model::SecurityMonitoringDatasetCreateDataRequest;
use datadog_api_client::datadogV2::model::SecurityMonitoringDatasetCreateRequest;
use datadog_api_client::datadogV2::model::SecurityMonitoringDatasetDefinition;
use datadog_api_client::datadogV2::model::SecurityMonitoringDatasetDefinitionColumn;
use datadog_api_client::datadogV2::model::SecurityMonitoringDatasetDefinitionColumnType;
use datadog_api_client::datadogV2::model::SecurityMonitoringDatasetType;

#[tokio::main]
async fn main() {
    let body = SecurityMonitoringDatasetCreateRequest::new(
        SecurityMonitoringDatasetCreateDataRequest::new(
            SecurityMonitoringDatasetCreateAttributesRequest::new(
                SecurityMonitoringDatasetDefinition::new(
                    vec![SecurityMonitoringDatasetDefinitionColumn::new(
                        "message".to_string(),
                        SecurityMonitoringDatasetDefinitionColumnType::STRING,
                    )],
                    "logs".to_string(),
                    "my_dataset".to_string(),
                )
                .indexes(vec!["k9".to_string()]),
                "A dataset for monitoring authentication events".to_string(),
            ),
            SecurityMonitoringDatasetType::SECURITY_MONITORING_DATASET,
        ),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateSecurityMonitoringDataset", true);
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api.create_security_monitoring_dataset(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
