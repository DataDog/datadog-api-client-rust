// List findings returns "OK" response with details
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::ListFindingsOptionalParams;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ListFindings", true);
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api
        .list_findings(ListFindingsOptionalParams::default().detailed_findings(true))
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
