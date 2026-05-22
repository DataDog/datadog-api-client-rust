// List LLM Observability dataset versions returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_llm_observability::LLMObservabilityAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ListLLMObsDatasetVersions", true);
    let api = LLMObservabilityAPI::with_config(configuration);
    let resp = api
        .list_llm_obs_dataset_versions("project_id".to_string(), "dataset_id".to_string())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
