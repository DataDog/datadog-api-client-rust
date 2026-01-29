// Update a dataset returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::SecurityMonitoringDatasetDefinition;
use datadog_api_client::datadogV2::model::SecurityMonitoringDatasetDefinitionColumn;
use datadog_api_client::datadogV2::model::SecurityMonitoringDatasetDefinitionColumnType;
use datadog_api_client::datadogV2::model::SecurityMonitoringDatasetType;
use datadog_api_client::datadogV2::model::SecurityMonitoringDatasetUpdateAttributesRequest;
use datadog_api_client::datadogV2::model::SecurityMonitoringDatasetUpdateDataRequest;
use datadog_api_client::datadogV2::model::SecurityMonitoringDatasetUpdateRequest;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body = SecurityMonitoringDatasetUpdateRequest::new(
        SecurityMonitoringDatasetUpdateDataRequest::new(
            SecurityMonitoringDatasetUpdateAttributesRequest::new(
                SecurityMonitoringDatasetDefinition::new(
                    vec![SecurityMonitoringDatasetDefinitionColumn::new(
                        "message".to_string(),
                        SecurityMonitoringDatasetDefinitionColumnType::STRING,
                    )],
                    "logs".to_string(),
                    "my_dataset".to_string(),
                )
                .indexes(vec!["k9".to_string()]),
                "Updated dataset description".to_string(),
            )
            .version(1),
            SecurityMonitoringDatasetType::SECURITY_MONITORING_DATASET,
        ),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdateSecurityMonitoringDataset", true);
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api
        .update_security_monitoring_dataset(
            Uuid::parse_str("123e4567-e89b-12d3-a456-426614174000").expect("invalid UUID"),
            body,
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
