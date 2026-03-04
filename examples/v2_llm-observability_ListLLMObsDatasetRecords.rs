// List LLM Observability dataset records returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_llm_observability::LLMObservabilityAPI;
use datadog_api_client::datadogV2::api_llm_observability::ListLLMObsDatasetRecordsOptionalParams;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ListLLMObsDatasetRecords", true);
    let api = LLMObservabilityAPI::with_config(configuration);
    let resp = api
        .list_llm_obs_dataset_records(
            "project_id".to_string(),
            "dataset_id".to_string(),
            ListLLMObsDatasetRecordsOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
