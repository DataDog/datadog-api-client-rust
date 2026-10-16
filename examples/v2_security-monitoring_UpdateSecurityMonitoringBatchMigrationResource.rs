// Update a batch migration resource returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::SecurityMonitoringBatchMigrationResourceType;
use datadog_api_client::datadogV2::model::SecurityMonitoringBatchMigrationResourceUpdateAttributes;
use datadog_api_client::datadogV2::model::SecurityMonitoringBatchMigrationResourceUpdateData;
use datadog_api_client::datadogV2::model::SecurityMonitoringBatchMigrationResourceUpdateRequest;

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
    let body = SecurityMonitoringBatchMigrationResourceUpdateRequest::new(
        SecurityMonitoringBatchMigrationResourceUpdateData::new(
            SecurityMonitoringBatchMigrationResourceUpdateAttributes::new()
                .input("index=wineventlog EventCode=4625 | stats count by user".to_string()),
            SecurityMonitoringBatchMigrationResourceType::BATCH_MIGRATION_RESOURCE,
        ),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled(
        "v2.update_security_monitoring_batch_migration_resource",
        true,
    );
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api
        .update_security_monitoring_batch_migration_resource(
            batch_migration_data_id.clone(),
            batch_migration_resource_id.clone(),
            body,
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
