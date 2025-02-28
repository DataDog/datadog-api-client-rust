// List vulnerable assets returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::ListVulnerableAssetsOptionalParams;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::AssetType;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ListVulnerableAssets", true);
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api
        .list_vulnerable_assets(
            ListVulnerableAssetsOptionalParams::default()
                .filter_type(AssetType::HOST)
                .filter_repository_url("github.com/datadog/dd-go".to_string())
                .filter_risks_in_production(true),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
