// Delete an LLM Observability annotation queue returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_llm_observability::LLMObservabilityAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.DeleteLLMObsAnnotationQueue", true);
    let api = LLMObservabilityAPI::with_config(configuration);
    let resp = api
        .delete_llm_obs_annotation_queue("queue_id".to_string())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
