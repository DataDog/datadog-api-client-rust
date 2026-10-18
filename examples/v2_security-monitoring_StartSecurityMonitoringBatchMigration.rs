// Start a batch migration returns "Accepted" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration
        .set_unstable_operation_enabled("v2.start_security_monitoring_batch_migration", true);
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api
        .start_security_monitoring_batch_migration(
            Uuid::parse_str("7cdb3a75-64fb-46ce-9445-41969e78fe8e").expect("invalid UUID"),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
