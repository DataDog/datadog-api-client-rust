// Get the CSM Serverless Coverage Analysis returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_csm_coverage_analysis::CSMCoverageAnalysisAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = CSMCoverageAnalysisAPI::with_config(configuration);
    let resp = api.get_csm_serverless_coverage_analysis().await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
