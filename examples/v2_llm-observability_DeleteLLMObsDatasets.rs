// Delete LLM Observability datasets returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_llm_observability::LLMObservabilityAPI;
use datadog_api_client::datadogV2::model::LLMObsDatasetType;
use datadog_api_client::datadogV2::model::LLMObsDeleteDatasetsDataAttributesRequest;
use datadog_api_client::datadogV2::model::LLMObsDeleteDatasetsDataRequest;
use datadog_api_client::datadogV2::model::LLMObsDeleteDatasetsRequest;

#[tokio::main]
async fn main() {
    let body = LLMObsDeleteDatasetsRequest::new(LLMObsDeleteDatasetsDataRequest::new(
        LLMObsDeleteDatasetsDataAttributesRequest::new(vec![
            "9f64e5c7-dc5a-45c8-a17c-1b85f0bec97d".to_string(),
        ]),
        LLMObsDatasetType::DATASETS,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.DeleteLLMObsDatasets", true);
    let api = LLMObservabilityAPI::with_config(configuration);
    let resp = api
        .delete_llm_obs_datasets("project_id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
