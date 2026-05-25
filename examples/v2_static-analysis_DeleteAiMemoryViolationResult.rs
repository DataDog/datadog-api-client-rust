// Delete an AI memory violation result returns "Successfully deleted" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_static_analysis::StaticAnalysisAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.DeleteAiMemoryViolationResult", true);
    let api = StaticAnalysisAPI::with_config(configuration);
    let resp = api
        .delete_ai_memory_violation_result("42".to_string())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
