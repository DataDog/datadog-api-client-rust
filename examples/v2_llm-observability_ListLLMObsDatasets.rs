// List LLM Observability datasets returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_llm_observability::LLMObservabilityAPI;
use datadog_api_client::datadogV2::api_llm_observability::ListLLMObsDatasetsOptionalParams;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ListLLMObsDatasets", true);
    let api = LLMObservabilityAPI::with_config(configuration);
    let resp = api
        .list_llm_obs_datasets(
            "project_id".to_string(),
            ListLLMObsDatasetsOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
