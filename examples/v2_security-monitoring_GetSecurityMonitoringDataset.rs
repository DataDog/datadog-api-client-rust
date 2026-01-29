// Get a dataset returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.GetSecurityMonitoringDataset", true);
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api
        .get_security_monitoring_dataset(
            Uuid::parse_str("123e4567-e89b-12d3-a456-426614174000").expect("invalid UUID"),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
