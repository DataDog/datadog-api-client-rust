// Add resources to a batch migration returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::SecurityMonitoringBatchMigrationResourceCreateItem;
use datadog_api_client::datadogV2::model::SecurityMonitoringBatchMigrationResourceInputType;
use datadog_api_client::datadogV2::model::SecurityMonitoringBatchMigrationResourcesCreateAttributes;
use datadog_api_client::datadogV2::model::SecurityMonitoringBatchMigrationResourcesCreateData;
use datadog_api_client::datadogV2::model::SecurityMonitoringBatchMigrationResourcesCreateRequest;
use datadog_api_client::datadogV2::model::SecurityMonitoringBatchMigrationResourcesType;

#[tokio::main]
async fn main() {
    // there is a valid "batch_migration" in the system
    let batch_migration_data_id =
        uuid::Uuid::parse_str(&std::env::var("BATCH_MIGRATION_DATA_ID").unwrap())
            .expect("Invalid UUID");
    let body =
        SecurityMonitoringBatchMigrationResourcesCreateRequest::new(
            SecurityMonitoringBatchMigrationResourcesCreateData::new(
                SecurityMonitoringBatchMigrationResourcesCreateAttributes::new(
                    vec![
                        SecurityMonitoringBatchMigrationResourceCreateItem::new(
                            r#"index=linux process=sudo "authentication failure" | stats count by user, host"#.to_string(),
                            SecurityMonitoringBatchMigrationResourceInputType::SPL,
                        ).name("Linux sudo abuse".to_string())
                    ],
                ),
                SecurityMonitoringBatchMigrationResourcesType::BATCH_MIGRATION_RESOURCES,
            ),
        );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled(
        "v2.create_security_monitoring_batch_migration_resources",
        true,
    );
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api
        .create_security_monitoring_batch_migration_resources(batch_migration_data_id.clone(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
