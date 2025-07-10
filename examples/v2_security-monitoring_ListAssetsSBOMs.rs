// List assets SBOMs returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::ListAssetsSBOMsOptionalParams;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::AssetType;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ListAssetsSBOMs", true);
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api
        .list_assets_sbo_ms(
            ListAssetsSBOMsOptionalParams::default()
                .filter_package_name("pandas".to_string())
                .filter_asset_type(AssetType::SERVICE),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
