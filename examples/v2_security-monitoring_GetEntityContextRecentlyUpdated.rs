// Get recently updated entity context returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::GetEntityContextRecentlyUpdatedOptionalParams;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.get_entity_context_recently_updated", true);
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api
        .get_entity_context_recently_updated(
            GetEntityContextRecentlyUpdatedOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
