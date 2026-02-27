// Update an LLM Observability dataset returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_llm_observability::LLMObservabilityAPI;
use datadog_api_client::datadogV2::model::LLMObsDatasetType;
use datadog_api_client::datadogV2::model::LLMObsDatasetUpdateDataAttributesRequest;
use datadog_api_client::datadogV2::model::LLMObsDatasetUpdateDataRequest;
use datadog_api_client::datadogV2::model::LLMObsDatasetUpdateRequest;

#[tokio::main]
async fn main() {
    let body = LLMObsDatasetUpdateRequest::new(LLMObsDatasetUpdateDataRequest::new(
        LLMObsDatasetUpdateDataAttributesRequest::new(),
        LLMObsDatasetType::DATASETS,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdateLLMObsDataset", true);
    let api = LLMObservabilityAPI::with_config(configuration);
    let resp = api
        .update_llm_obs_dataset("project_id".to_string(), "dataset_id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
