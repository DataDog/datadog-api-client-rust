// Create a batch migration returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::SecurityMonitoringBatchMigrationCreateAttributes;
use datadog_api_client::datadogV2::model::SecurityMonitoringBatchMigrationCreateData;
use datadog_api_client::datadogV2::model::SecurityMonitoringBatchMigrationCreateRequest;
use datadog_api_client::datadogV2::model::SecurityMonitoringBatchMigrationResourceCreateItem;
use datadog_api_client::datadogV2::model::SecurityMonitoringBatchMigrationResourceInputType;
use datadog_api_client::datadogV2::model::SecurityMonitoringBatchMigrationType;

#[tokio::main]
async fn main() {
    let body = SecurityMonitoringBatchMigrationCreateRequest::new(
        SecurityMonitoringBatchMigrationCreateData::new(
            SecurityMonitoringBatchMigrationCreateAttributes::new(vec![
                SecurityMonitoringBatchMigrationResourceCreateItem::new(
                    "index=wineventlog EventCode=4625".to_string(),
                    SecurityMonitoringBatchMigrationResourceInputType::SPL,
                )
                .description("Detects repeated failed Windows authentication attempts.".to_string())
                .name("Repeated Windows login failures".to_string()),
            ])
            .description("Rules imported from Splunk".to_string())
            .name("Example-Security-Monitoring".to_string())
            .tags(vec!["source:splunk".to_string()]),
            SecurityMonitoringBatchMigrationType::BATCH_MIGRATION,
        ),
    );
    let mut configuration = datadog::Configuration::new();
    configuration
        .set_unstable_operation_enabled("v2.create_security_monitoring_batch_migration", true);
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api.create_security_monitoring_batch_migration(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
