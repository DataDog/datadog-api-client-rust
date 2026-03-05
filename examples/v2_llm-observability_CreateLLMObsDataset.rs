// Create an LLM Observability dataset returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_llm_observability::LLMObservabilityAPI;
use datadog_api_client::datadogV2::model::LLMObsDatasetDataAttributesRequest;
use datadog_api_client::datadogV2::model::LLMObsDatasetDataRequest;
use datadog_api_client::datadogV2::model::LLMObsDatasetRequest;
use datadog_api_client::datadogV2::model::LLMObsDatasetType;

#[tokio::main]
async fn main() {
    let body = LLMObsDatasetRequest::new(LLMObsDatasetDataRequest::new(
        LLMObsDatasetDataAttributesRequest::new("My LLM Dataset".to_string()),
        LLMObsDatasetType::DATASETS,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateLLMObsDataset", true);
    let api = LLMObservabilityAPI::with_config(configuration);
    let resp = api
        .create_llm_obs_dataset("project_id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
