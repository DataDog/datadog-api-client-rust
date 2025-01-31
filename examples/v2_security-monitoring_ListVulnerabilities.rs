// List vulnerabilities returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::ListVulnerabilitiesOptionalParams;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::AssetType;
use datadog_api_client::datadogV2::model::VulnerabilitySeverity;
use datadog_api_client::datadogV2::model::VulnerabilityTool;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api
        .list_vulnerabilities(
            ListVulnerabilitiesOptionalParams::default()
                .filter_cvss_base_severity(VulnerabilitySeverity::HIGH)
                .filter_asset_type(AssetType::SERVICE)
                .filter_tool(VulnerabilityTool::INFRA),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
