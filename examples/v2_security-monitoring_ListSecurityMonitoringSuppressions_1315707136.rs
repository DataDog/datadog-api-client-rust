// Get all suppression rules returns "OK" response with sort ascending
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::ListSecurityMonitoringSuppressionsOptionalParams;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::SecurityMonitoringSuppressionSort;

#[tokio::main]
async fn main() {
    // there is a valid "suppression" in the system there is a valid "suppression2" in
    // the system
    let configuration = datadog::Configuration::new();
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api
        .list_security_monitoring_suppressions(
            ListSecurityMonitoringSuppressionsOptionalParams::default()
                .sort(SecurityMonitoringSuppressionSort::NAME)
                .query("id:3dd-0uc-h1s OR id:886e6c3e-e543-049c-ee1b-56a1110295c0".to_string()),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
