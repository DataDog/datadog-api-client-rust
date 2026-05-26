// Create a dataset returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::SecurityMonitoringDatasetAttributesRequest;
use datadog_api_client::datadogV2::model::SecurityMonitoringDatasetColumn;
use datadog_api_client::datadogV2::model::SecurityMonitoringDatasetCreateData;
use datadog_api_client::datadogV2::model::SecurityMonitoringDatasetCreateRequest;
use datadog_api_client::datadogV2::model::SecurityMonitoringDatasetCreateType;
use datadog_api_client::datadogV2::model::SecurityMonitoringDatasetDefinition;
use datadog_api_client::datadogV2::model::SecurityMonitoringDatasetSearch;
use datadog_api_client::datadogV2::model::SecurityMonitoringDatasetTimeWindow;

#[tokio::main]
async fn main() {
    let body =
        SecurityMonitoringDatasetCreateRequest::new(SecurityMonitoringDatasetCreateData::new(
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
            SecurityMonitoringDatasetCreateType::DATASET_CREATE,
        ));
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
