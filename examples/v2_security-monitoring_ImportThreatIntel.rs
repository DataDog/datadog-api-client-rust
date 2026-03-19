// Import threat intelligence feed returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::ImportThreatIntelOptionalParams;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::ThreatIntelIndicatorType;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ImportThreatIntel", true);
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api
        .import_threat_intel(
            "ti_vendor".to_string(),
            ThreatIntelIndicatorType::IP_ADDRESS,
            ImportThreatIntelOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
