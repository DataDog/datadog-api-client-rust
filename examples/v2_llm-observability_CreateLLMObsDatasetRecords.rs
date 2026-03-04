// Append records to an LLM Observability dataset returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_llm_observability::LLMObservabilityAPI;
use datadog_api_client::datadogV2::model::LLMObsDatasetRecordItem;
use datadog_api_client::datadogV2::model::LLMObsDatasetRecordsDataAttributesRequest;
use datadog_api_client::datadogV2::model::LLMObsDatasetRecordsDataRequest;
use datadog_api_client::datadogV2::model::LLMObsDatasetRecordsRequest;
use datadog_api_client::datadogV2::model::LLMObsRecordType;

#[tokio::main]
async fn main() {
    let body = LLMObsDatasetRecordsRequest::new(LLMObsDatasetRecordsDataRequest::new(
        LLMObsDatasetRecordsDataAttributesRequest::new(vec![
            LLMObsDatasetRecordItem::new(None).expected_output(None)
        ]),
        LLMObsRecordType::RECORDS,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateLLMObsDatasetRecords", true);
    let api = LLMObservabilityAPI::with_config(configuration);
    let resp = api
        .create_llm_obs_dataset_records("project_id".to_string(), "dataset_id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
