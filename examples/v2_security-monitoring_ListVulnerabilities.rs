// List vulnerabilities returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::ListVulnerabilitiesOptionalParams;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::Tool;
use datadog_api_client::datadogV2::model::VulnerabilityType;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ListVulnerabilities", true);
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api
        .list_vulnerabilities(
            ListVulnerabilitiesOptionalParams::default()
                .filter_type(VulnerabilityType::COMMANDINJECTION)
                .filter_asset_repository_url("github.com/datadog/dd-go".to_string())
                .filter_tool(Tool::IAST),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
