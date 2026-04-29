// Update annotation queue label schema returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_llm_observability::LLMObservabilityAPI;
use datadog_api_client::datadogV2::model::LLMObsAnnotationQueueLabelSchemaUpdateAttributes;
use datadog_api_client::datadogV2::model::LLMObsAnnotationQueueLabelSchemaUpdateData;
use datadog_api_client::datadogV2::model::LLMObsAnnotationQueueLabelSchemaUpdateRequest;
use datadog_api_client::datadogV2::model::LLMObsAnnotationQueueType;
use datadog_api_client::datadogV2::model::LLMObsAnnotationSchema;
use datadog_api_client::datadogV2::model::LLMObsLabelSchema;
use datadog_api_client::datadogV2::model::LLMObsLabelSchemaType;

#[tokio::main]
async fn main() {
    let body = LLMObsAnnotationQueueLabelSchemaUpdateRequest::new(
        LLMObsAnnotationQueueLabelSchemaUpdateData::new(
            LLMObsAnnotationQueueLabelSchemaUpdateAttributes::new(LLMObsAnnotationSchema::new(
                vec![
                    LLMObsLabelSchema::new("quality".to_string(), LLMObsLabelSchemaType::SCORE)
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
                        ]),
                ],
            )),
            LLMObsAnnotationQueueType::QUEUES,
        ),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdateLLMObsAnnotationQueueLabelSchema", true);
    let api = LLMObservabilityAPI::with_config(configuration);
    let resp = api
        .update_llm_obs_annotation_queue_label_schema("queue_id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
