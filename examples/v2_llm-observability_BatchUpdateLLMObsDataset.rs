// Batch update LLM Observability dataset records returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_llm_observability::LLMObservabilityAPI;
use datadog_api_client::datadogV2::model::LLMObsDatasetBatchUpdateDataAttributesRequest;
use datadog_api_client::datadogV2::model::LLMObsDatasetBatchUpdateDataRequest;
use datadog_api_client::datadogV2::model::LLMObsDatasetBatchUpdateInsertRecord;
use datadog_api_client::datadogV2::model::LLMObsDatasetBatchUpdateRequest;
use datadog_api_client::datadogV2::model::LLMObsDatasetBatchUpdateUpdateRecord;
use datadog_api_client::datadogV2::model::LLMObsDatasetRecordTagOperations;
use datadog_api_client::datadogV2::model::LLMObsDatasetType;

#[tokio::main]
async fn main() {
    let body = LLMObsDatasetBatchUpdateRequest::new(LLMObsDatasetBatchUpdateDataRequest::new(
        LLMObsDatasetBatchUpdateDataAttributesRequest::new()
            .create_new_version(true)
            .delete_records(vec![])
            .insert_records(vec![LLMObsDatasetBatchUpdateInsertRecord::new(None)
                .expected_output(None)
                .id("rec-7c3f5a1b-9e2d-4f8a-b1c6-3d7e9f0a2b4c".to_string())
                .tag_operations(
                    LLMObsDatasetRecordTagOperations::new()
                        .add(vec![])
                        .remove(vec![])
                        .set(vec![]),
                )
                .tags(vec![])])
            .tags(vec![])
            .update_records(vec![LLMObsDatasetBatchUpdateUpdateRecord::new(
                "rec-7c3f5a1b-9e2d-4f8a-b1c6-3d7e9f0a2b4c".to_string(),
            )
            .expected_output(None)
            .input(None)
            .tag_operations(
                LLMObsDatasetRecordTagOperations::new()
                    .add(vec![])
                    .remove(vec![])
                    .set(vec![]),
            )]),
        "9f64e5c7-dc5a-45c8-a17c-1b85f0bec97d".to_string(),
        LLMObsDatasetType::DATASETS,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.BatchUpdateLLMObsDataset", true);
    let api = LLMObservabilityAPI::with_config(configuration);
    let resp = api
        .batch_update_llm_obs_dataset("project_id".to_string(), "dataset_id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
