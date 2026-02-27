// Delete LLM Observability dataset records returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_llm_observability::LLMObservabilityAPI;
use datadog_api_client::datadogV2::model::LLMObsDeleteDatasetRecordsDataAttributesRequest;
use datadog_api_client::datadogV2::model::LLMObsDeleteDatasetRecordsDataRequest;
use datadog_api_client::datadogV2::model::LLMObsDeleteDatasetRecordsRequest;
use datadog_api_client::datadogV2::model::LLMObsRecordType;

#[tokio::main]
async fn main() {
    let body = LLMObsDeleteDatasetRecordsRequest::new(LLMObsDeleteDatasetRecordsDataRequest::new(
        LLMObsDeleteDatasetRecordsDataAttributesRequest::new(vec![
            "rec-7c3f5a1b-9e2d-4f8a-b1c6-3d7e9f0a2b4c".to_string(),
        ]),
        LLMObsRecordType::RECORDS,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.DeleteLLMObsDatasetRecords", true);
    let api = LLMObservabilityAPI::with_config(configuration);
    let resp = api
        .delete_llm_obs_dataset_records("project_id".to_string(), "dataset_id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
