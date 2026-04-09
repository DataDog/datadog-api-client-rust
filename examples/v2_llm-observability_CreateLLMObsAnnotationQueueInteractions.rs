// Add annotation queue interactions returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_llm_observability::LLMObservabilityAPI;
use datadog_api_client::datadogV2::model::LLMObsAnnotationQueueInteractionItem;
use datadog_api_client::datadogV2::model::LLMObsAnnotationQueueInteractionsDataAttributesRequest;
use datadog_api_client::datadogV2::model::LLMObsAnnotationQueueInteractionsDataRequest;
use datadog_api_client::datadogV2::model::LLMObsAnnotationQueueInteractionsRequest;
use datadog_api_client::datadogV2::model::LLMObsAnnotationQueueInteractionsType;
use datadog_api_client::datadogV2::model::LLMObsInteractionType;

#[tokio::main]
async fn main() {
    let body = LLMObsAnnotationQueueInteractionsRequest::new(
        LLMObsAnnotationQueueInteractionsDataRequest::new(
            LLMObsAnnotationQueueInteractionsDataAttributesRequest::new(vec![
                LLMObsAnnotationQueueInteractionItem::new(
                    "trace-abc-123".to_string(),
                    LLMObsInteractionType::TRACE,
                ),
            ]),
            LLMObsAnnotationQueueInteractionsType::INTERACTIONS,
        ),
    );
    let mut configuration = datadog::Configuration::new();
    configuration
        .set_unstable_operation_enabled("v2.CreateLLMObsAnnotationQueueInteractions", true);
    let api = LLMObservabilityAPI::with_config(configuration);
    let resp = api
        .create_llm_obs_annotation_queue_interactions("queue_id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
