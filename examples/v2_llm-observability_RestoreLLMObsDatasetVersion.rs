// Restore an LLM Observability dataset version returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_llm_observability::LLMObservabilityAPI;
use datadog_api_client::datadogV2::model::LLMObsDatasetRestoreVersionDataAttributesRequest;
use datadog_api_client::datadogV2::model::LLMObsDatasetRestoreVersionDataRequest;
use datadog_api_client::datadogV2::model::LLMObsDatasetRestoreVersionRequest;
use datadog_api_client::datadogV2::model::LLMObsDatasetType;

#[tokio::main]
async fn main() {
    let body =
        LLMObsDatasetRestoreVersionRequest::new(LLMObsDatasetRestoreVersionDataRequest::new(
            LLMObsDatasetRestoreVersionDataAttributesRequest::new(1),
            "9f64e5c7-dc5a-45c8-a17c-1b85f0bec97d".to_string(),
            LLMObsDatasetType::DATASETS,
        ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.RestoreLLMObsDatasetVersion", true);
    let api = LLMObservabilityAPI::with_config(configuration);
    let resp = api
        .restore_llm_obs_dataset_version("project_id".to_string(), "dataset_id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
