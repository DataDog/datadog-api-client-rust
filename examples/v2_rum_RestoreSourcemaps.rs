// Restore source maps returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum::RestoreSourcemapsOptionalParams;
use datadog_api_client::datadogV2::api_rum::RUMAPI;
use datadog_api_client::datadogV2::model::SourcemapMapKind;
use datadog_api_client::datadogV2::model::SourcemapSymbolType;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.restore_sourcemaps", true);
    let api = RUMAPI::with_config(configuration);
    let resp = api
        .restore_sourcemaps(
            SourcemapMapKind::WASM,
            true,
            RestoreSourcemapsOptionalParams::default()
                .symbol_type(SourcemapSymbolType::SOURCEMAP)
                .filter_service(vec!["nonexistent-svc".to_string()])
                .filter_version(vec!["0.0.0-none".to_string()]),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
