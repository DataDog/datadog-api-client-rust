// Delete source maps returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum::DeleteSourcemapsOptionalParams;
use datadog_api_client::datadogV2::api_rum::RUMAPI;
use datadog_api_client::datadogV2::model::SourcemapMapKind;
use datadog_api_client::datadogV2::model::SourcemapSymbolType;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.delete_sourcemaps", true);
    let api = RUMAPI::with_config(configuration);
    let resp = api
        .delete_sourcemaps(
            SourcemapMapKind::WASM,
            true,
            DeleteSourcemapsOptionalParams::default()
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
