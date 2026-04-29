// Update an LLM Observability annotation queue returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_llm_observability::LLMObservabilityAPI;
use datadog_api_client::datadogV2::model::LLMObsAnnotationQueueType;
use datadog_api_client::datadogV2::model::LLMObsAnnotationQueueUpdateDataAttributesRequest;
use datadog_api_client::datadogV2::model::LLMObsAnnotationQueueUpdateDataRequest;
use datadog_api_client::datadogV2::model::LLMObsAnnotationQueueUpdateRequest;
use datadog_api_client::datadogV2::model::LLMObsAnnotationSchema;
use datadog_api_client::datadogV2::model::LLMObsLabelSchema;
use datadog_api_client::datadogV2::model::LLMObsLabelSchemaType;

#[tokio::main]
async fn main() {
    let body =
        LLMObsAnnotationQueueUpdateRequest::new(LLMObsAnnotationQueueUpdateDataRequest::new(
            LLMObsAnnotationQueueUpdateDataAttributesRequest::new()
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
                .description("Updated description".to_string())
                .name("Updated queue name".to_string()),
            LLMObsAnnotationQueueType::QUEUES,
        ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdateLLMObsAnnotationQueue", true);
    let api = LLMObservabilityAPI::with_config(configuration);
    let resp = api
        .update_llm_obs_annotation_queue("queue_id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
