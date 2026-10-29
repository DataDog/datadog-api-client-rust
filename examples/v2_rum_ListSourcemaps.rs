// List source maps returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum::ListSourcemapsOptionalParams;
use datadog_api_client::datadogV2::api_rum::RUMAPI;
use datadog_api_client::datadogV2::model::SourcemapMapKind;
use datadog_api_client::datadogV2::model::SourcemapSymbolType;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.list_sourcemaps", true);
    let api = RUMAPI::with_config(configuration);
    let resp = api
        .list_sourcemaps(
            ListSourcemapsOptionalParams::default()
                .mapkind(SourcemapMapKind::WASM)
                .symbol_type(SourcemapSymbolType::SOURCEMAP)
                .filter_service(vec!["my-wasm-app".to_string()])
                .filter_version(vec!["1.0.0".to_string()]),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
