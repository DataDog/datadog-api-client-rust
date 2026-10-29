// Export feature flag change history returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_feature_flags::ExportFeatureFlagHistoryOptionalParams;
use datadog_api_client::datadogV2::api_feature_flags::FeatureFlagsAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = FeatureFlagsAPI::with_config(configuration);
    let resp = api
        .export_feature_flag_history(ExportFeatureFlagHistoryOptionalParams::default())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
