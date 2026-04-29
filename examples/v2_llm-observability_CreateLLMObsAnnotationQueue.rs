// Create an LLM Observability annotation queue returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_llm_observability::LLMObservabilityAPI;
use datadog_api_client::datadogV2::model::LLMObsAnnotationQueueDataAttributesRequest;
use datadog_api_client::datadogV2::model::LLMObsAnnotationQueueDataRequest;
use datadog_api_client::datadogV2::model::LLMObsAnnotationQueueRequest;
use datadog_api_client::datadogV2::model::LLMObsAnnotationQueueType;
use datadog_api_client::datadogV2::model::LLMObsAnnotationSchema;
use datadog_api_client::datadogV2::model::LLMObsLabelSchema;
use datadog_api_client::datadogV2::model::LLMObsLabelSchemaType;

#[tokio::main]
async fn main() {
    let body = LLMObsAnnotationQueueRequest::new(LLMObsAnnotationQueueDataRequest::new(
        LLMObsAnnotationQueueDataAttributesRequest::new(
            "My annotation queue".to_string(),
            "a33671aa-24fd-4dcd-9b33-a8ec7dde7751".to_string(),
        )
        .annotation_schema(LLMObsAnnotationSchema::new(vec![LLMObsLabelSchema::new(
            "quality".to_string(),
            LLMObsLabelSchemaType::SCORE,
        )
        .description("Rating of the response quality.".to_string())
        .has_assessment(false)
        .has_reasoning(false)
        .id("ab12cd34".to_string())
        .is_assessment(false)
        .is_integer(false)
        .is_required(true)
        .max(5.0 as f64)
        .min(0.0 as f64)
        .values(vec![
            "good".to_string(),
            "bad".to_string(),
            "neutral".to_string(),
        ])]))
        .description("Queue for annotating customer support traces".to_string()),
        LLMObsAnnotationQueueType::QUEUES,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateLLMObsAnnotationQueue", true);
    let api = LLMObservabilityAPI::with_config(configuration);
    let resp = api.create_llm_obs_annotation_queue(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
