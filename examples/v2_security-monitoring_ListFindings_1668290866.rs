// List findings with detection_type query param returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::ListFindingsOptionalParams;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::FindingVulnerabilityType;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ListFindings", true);
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api
        .list_findings(
            ListFindingsOptionalParams::default().filter_vulnerability_type(vec![
                FindingVulnerabilityType::MISCONFIGURATION,
                FindingVulnerabilityType::ATTACK_PATH,
            ]),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
