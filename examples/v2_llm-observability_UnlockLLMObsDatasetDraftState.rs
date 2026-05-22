// Unlock LLM Observability dataset draft state returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_llm_observability::LLMObservabilityAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UnlockLLMObsDatasetDraftState", true);
    let api = LLMObservabilityAPI::with_config(configuration);
    let resp = api
        .unlock_llm_obs_dataset_draft_state("project_id".to_string(), "dataset_id".to_string())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
