// Delete resources from a batch migration returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::SecurityMonitoringBatchMigrationResourcesDeleteAttributes;
use datadog_api_client::datadogV2::model::SecurityMonitoringBatchMigrationResourcesDeleteData;
use datadog_api_client::datadogV2::model::SecurityMonitoringBatchMigrationResourcesDeleteRequest;
use datadog_api_client::datadogV2::model::SecurityMonitoringBatchMigrationResourcesType;

#[tokio::main]
async fn main() {
    // there is a valid "batch_migration" in the system
    let batch_migration_data_id =
        uuid::Uuid::parse_str(&std::env::var("BATCH_MIGRATION_DATA_ID").unwrap())
            .expect("Invalid UUID");

    // there is a valid "batch_migration_resource" in the system
    let batch_migration_resource_id =
        uuid::Uuid::parse_str(&std::env::var("BATCH_MIGRATION_RESOURCE_ID").unwrap())
            .expect("Invalid UUID");
    let body = SecurityMonitoringBatchMigrationResourcesDeleteRequest::new(
        SecurityMonitoringBatchMigrationResourcesDeleteData::new(
            SecurityMonitoringBatchMigrationResourcesDeleteAttributes::new(vec![
                batch_migration_resource_id.clone(),
            ]),
            SecurityMonitoringBatchMigrationResourcesType::BATCH_MIGRATION_RESOURCES,
        ),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled(
        "v2.delete_security_monitoring_batch_migration_resources",
        true,
    );
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api
        .delete_security_monitoring_batch_migration_resources(batch_migration_data_id.clone(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
