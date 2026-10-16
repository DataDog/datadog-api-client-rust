// List batch migration resources returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::ListSecurityMonitoringBatchMigrationResourcesOptionalParams;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;

#[tokio::main]
async fn main() {
    // there is a valid "batch_migration" in the system
    let batch_migration_data_id =
        uuid::Uuid::parse_str(&std::env::var("BATCH_MIGRATION_DATA_ID").unwrap())
            .expect("Invalid UUID");
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled(
        "v2.list_security_monitoring_batch_migration_resources",
        true,
    );
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api
        .list_security_monitoring_batch_migration_resources(
            batch_migration_data_id.clone(),
            ListSecurityMonitoringBatchMigrationResourcesOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
