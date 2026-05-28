// Upload records to an LLM Observability dataset returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_llm_observability::LLMObservabilityAPI;
use datadog_api_client::datadogV2::api_llm_observability::UploadLLMObsDatasetRecordsFileOptionalParams;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UploadLLMObsDatasetRecordsFile", true);
    let api = LLMObservabilityAPI::with_config(configuration);
    let resp = api
        .upload_llm_obs_dataset_records_file(
            "project_id".to_string(),
            "dataset_id".to_string(),
            UploadLLMObsDatasetRecordsFileOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
