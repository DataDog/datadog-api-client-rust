// Publish a batch migration returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::SecurityMonitoringBatchMigrationPublishAttributes;
use datadog_api_client::datadogV2::model::SecurityMonitoringBatchMigrationPublishData;
use datadog_api_client::datadogV2::model::SecurityMonitoringBatchMigrationPublishRequest;
use datadog_api_client::datadogV2::model::SecurityMonitoringBatchMigrationPublishType;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body = SecurityMonitoringBatchMigrationPublishRequest::new(
        SecurityMonitoringBatchMigrationPublishData::new(
            SecurityMonitoringBatchMigrationPublishAttributes::new().resource_ids(vec![
                Uuid::parse_str("c968cf83-56c2-4d54-a993-6c86dfb83527").expect("invalid UUID"),
            ]),
            SecurityMonitoringBatchMigrationPublishType::BATCH_MIGRATION_PUBLISH,
        ),
    );
    let mut configuration = datadog::Configuration::new();
    configuration
        .set_unstable_operation_enabled("v2.publish_security_monitoring_batch_migration", true);
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api
        .publish_security_monitoring_batch_migration(
            Uuid::parse_str("7cdb3a75-64fb-46ce-9445-41969e78fe8e").expect("invalid UUID"),
            body,
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
