// List batch migrations returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::ListSecurityMonitoringBatchMigrationsOptionalParams;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;

#[tokio::main]
async fn main() {
    // there is a valid "batch_migration" in the system
    let mut configuration = datadog::Configuration::new();
    configuration
        .set_unstable_operation_enabled("v2.list_security_monitoring_batch_migrations", true);
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api
        .list_security_monitoring_batch_migrations(
            ListSecurityMonitoringBatchMigrationsOptionalParams::default()
                .filter_query("id:7cdb3a75-64fb-46ce-9445-41969e78fe8e".to_string()),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
