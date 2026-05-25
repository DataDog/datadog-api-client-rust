// Get tree-sitter WASM file returns "BLOB with the content of the WASM file"
// response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.GetStaticAnalysisTreeSitterWasm", true);
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api
        .get_static_analysis_tree_sitter_wasm("tree-sitter-python.wasm".to_string())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
