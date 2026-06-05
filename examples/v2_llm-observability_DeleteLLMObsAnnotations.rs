// Delete annotations returns "OK — annotations deleted. Errors for annotations
// that could not be deleted are listed in `errors`." response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_llm_observability::LLMObservabilityAPI;
use datadog_api_client::datadogV2::model::LLMObsAnnotationsType;
use datadog_api_client::datadogV2::model::LLMObsDeleteAnnotationsDataAttributesRequest;
use datadog_api_client::datadogV2::model::LLMObsDeleteAnnotationsDataRequest;
use datadog_api_client::datadogV2::model::LLMObsDeleteAnnotationsRequest;

#[tokio::main]
async fn main() {
    let body = LLMObsDeleteAnnotationsRequest::new(LLMObsDeleteAnnotationsDataRequest::new(
        LLMObsDeleteAnnotationsDataAttributesRequest::new(vec![
            "00000000-0000-0000-0000-000000000000".to_string(),
            "00000000-0000-0000-0000-000000000001".to_string(),
        ]),
        LLMObsAnnotationsType::ANNOTATIONS,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.DeleteLLMObsAnnotations", true);
    let api = LLMObservabilityAPI::with_config(configuration);
    let resp = api
        .delete_llm_obs_annotations("queue_id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
