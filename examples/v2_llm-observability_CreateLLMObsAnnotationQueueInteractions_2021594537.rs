// Add a display_block interaction returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_llm_observability::LLMObservabilityAPI;
use datadog_api_client::datadogV2::model::LLMObsAnnotationQueueInteractionItem;
use datadog_api_client::datadogV2::model::LLMObsAnnotationQueueInteractionsDataAttributesRequest;
use datadog_api_client::datadogV2::model::LLMObsAnnotationQueueInteractionsDataRequest;
use datadog_api_client::datadogV2::model::LLMObsAnnotationQueueInteractionsRequest;
use datadog_api_client::datadogV2::model::LLMObsAnnotationQueueInteractionsType;
use datadog_api_client::datadogV2::model::LLMObsContentBlock;
use datadog_api_client::datadogV2::model::LLMObsContentBlockType;
use datadog_api_client::datadogV2::model::LLMObsDisplayBlockInteractionItem;
use datadog_api_client::datadogV2::model::LLMObsDisplayBlockInteractionType;
use serde_json::Value;

#[tokio::main]
async fn main() {
    let body = LLMObsAnnotationQueueInteractionsRequest::new(
        LLMObsAnnotationQueueInteractionsDataRequest::new(
            LLMObsAnnotationQueueInteractionsDataAttributesRequest::new(vec![
                LLMObsAnnotationQueueInteractionItem::LLMObsDisplayBlockInteractionItem(Box::new(
                    LLMObsDisplayBlockInteractionItem::new(
                        vec![LLMObsContentBlock::new(LLMObsContentBlockType::MARKDOWN)
                            .content(Value::from("## Triage Instructions"))],
                        LLMObsDisplayBlockInteractionType::DISPLAY_BLOCK,
                    ),
                )),
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
