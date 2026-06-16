// Trigger a patterns run returns "Accepted" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_llm_observability::LLMObservabilityAPI;
use datadog_api_client::datadogV2::model::LLMObsPatternsRequestType;
use datadog_api_client::datadogV2::model::LLMObsPatternsTriggerRequest;
use datadog_api_client::datadogV2::model::LLMObsPatternsTriggerRequestAttributes;
use datadog_api_client::datadogV2::model::LLMObsPatternsTriggerRequestData;

#[tokio::main]
async fn main() {
    let body = LLMObsPatternsTriggerRequest::new(LLMObsPatternsTriggerRequestData::new(
        LLMObsPatternsTriggerRequestAttributes::new(
            "a7c8d9e0-1234-5678-9abc-def012345678".to_string(),
        ),
        LLMObsPatternsRequestType::TOPIC_DISCOVERY,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.TriggerLLMObsPatterns", true);
    let api = LLMObservabilityAPI::with_config(configuration);
    let resp = api.trigger_llm_obs_patterns(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
