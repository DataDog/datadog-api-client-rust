// Update a dataset returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::SecurityMonitoringDatasetAttributesRequest;
use datadog_api_client::datadogV2::model::SecurityMonitoringDatasetColumn;
use datadog_api_client::datadogV2::model::SecurityMonitoringDatasetDefinition;
use datadog_api_client::datadogV2::model::SecurityMonitoringDatasetSearch;
use datadog_api_client::datadogV2::model::SecurityMonitoringDatasetTimeWindow;
use datadog_api_client::datadogV2::model::SecurityMonitoringDatasetUpdateData;
use datadog_api_client::datadogV2::model::SecurityMonitoringDatasetUpdateRequest;
use datadog_api_client::datadogV2::model::SecurityMonitoringDatasetUpdateType;

#[tokio::main]
async fn main() {
    let body =
        SecurityMonitoringDatasetUpdateRequest::new(SecurityMonitoringDatasetUpdateData::new(
            SecurityMonitoringDatasetAttributesRequest::new(
                SecurityMonitoringDatasetDefinition::new(
                    "logs".to_string(),
                    "sample_dataset".to_string(),
                )
                .columns(vec![SecurityMonitoringDatasetColumn::new(
                    "message".to_string(),
                    "string".to_string(),
                )])
                .indexes(vec![])
                .query_filter("status = 'active'".to_string())
                .search(SecurityMonitoringDatasetSearch::new("*".to_string()))
                .storage("hot".to_string())
                .table_name("my_reference_table".to_string())
                .time_window(
                    SecurityMonitoringDatasetTimeWindow::new()
                        .from(1700000000000)
                        .to(1700003600000),
                ),
            )
            .description("A sample dataset used for detection rules.".to_string())
            .version(1),
            SecurityMonitoringDatasetUpdateType::DATASET_UPDATE,
        ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdateSecurityMonitoringDataset", true);
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api
        .update_security_monitoring_dataset(
            "123e4567-e89b-12d3-a456-426614174000".to_string(),
            body,
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
