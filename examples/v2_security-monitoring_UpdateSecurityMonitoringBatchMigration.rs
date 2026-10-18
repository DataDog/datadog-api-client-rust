// Update a batch migration returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::SecurityMonitoringBatchMigrationType;
use datadog_api_client::datadogV2::model::SecurityMonitoringBatchMigrationUpdateAttributes;
use datadog_api_client::datadogV2::model::SecurityMonitoringBatchMigrationUpdateData;
use datadog_api_client::datadogV2::model::SecurityMonitoringBatchMigrationUpdateRequest;

#[tokio::main]
async fn main() {
    // there is a valid "batch_migration" in the system
    let batch_migration_data_id =
        uuid::Uuid::parse_str(&std::env::var("BATCH_MIGRATION_DATA_ID").unwrap())
            .expect("Invalid UUID");
    let body = SecurityMonitoringBatchMigrationUpdateRequest::new(
        SecurityMonitoringBatchMigrationUpdateData::new(
            SecurityMonitoringBatchMigrationUpdateAttributes::new()
                .name("Example-Security-Monitoring-updated".to_string())
                .tags(vec!["source:splunk".to_string(), "team:secops".to_string()]),
            SecurityMonitoringBatchMigrationType::BATCH_MIGRATION,
        ),
    );
    let mut configuration = datadog::Configuration::new();
    configuration
        .set_unstable_operation_enabled("v2.update_security_monitoring_batch_migration", true);
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api
        .update_security_monitoring_batch_migration(batch_migration_data_id.clone(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
