// Retrieve a dependency scan result returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_static_analysis::StaticAnalysisAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.GetSCAScan", true);
    let api = StaticAnalysisAPI::with_config(configuration);
    let resp = api
        .get_sca_scan("0190a3d4-1234-7000-8000-000000000000".to_string())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
