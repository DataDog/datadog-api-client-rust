// Create or update annotations returns "OK — annotations created or updated.
// Per-item errors are listed in `errors`." response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_llm_observability::LLMObservabilityAPI;
use datadog_api_client::datadogV2::model::LLMObsAnnotationLabelValue;
use datadog_api_client::datadogV2::model::LLMObsAnnotationLabelValueValue;
use datadog_api_client::datadogV2::model::LLMObsAnnotationsDataAttributesRequest;
use datadog_api_client::datadogV2::model::LLMObsAnnotationsDataRequest;
use datadog_api_client::datadogV2::model::LLMObsAnnotationsRequest;
use datadog_api_client::datadogV2::model::LLMObsAnnotationsType;
use datadog_api_client::datadogV2::model::LLMObsUpsertAnnotationItem;

#[tokio::main]
async fn main() {
    let body = LLMObsAnnotationsRequest::new(LLMObsAnnotationsDataRequest::new(
        LLMObsAnnotationsDataAttributesRequest::new(vec![LLMObsUpsertAnnotationItem::new(
            "00000000-0000-0000-0000-000000000001".to_string(),
            vec![
                LLMObsAnnotationLabelValue::new(
                    "abc-123".to_string(),
                    LLMObsAnnotationLabelValueValue::AnyValueString("good".to_string()),
                ),
                LLMObsAnnotationLabelValue::new(
                    "ef56gh78".to_string(),
                    LLMObsAnnotationLabelValueValue::AnyValueString("positive".to_string()),
                ),
            ],
        )]),
        LLMObsAnnotationsType::ANNOTATIONS,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpsertLLMObsAnnotations", true);
    let api = LLMObservabilityAPI::with_config(configuration);
    let resp = api
        .upsert_llm_obs_annotations("queue_id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
