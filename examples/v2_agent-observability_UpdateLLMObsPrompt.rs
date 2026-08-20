// Update an Agent Observability prompt returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_agent_observability::AgentObservabilityAPI;
use datadog_api_client::datadogV2::model::LLMObsPromptType;
use datadog_api_client::datadogV2::model::LLMObsUpdatePromptData;
use datadog_api_client::datadogV2::model::LLMObsUpdatePromptDataAttributes;
use datadog_api_client::datadogV2::model::LLMObsUpdatePromptRequest;

#[tokio::main]
async fn main() {
    let body = LLMObsUpdatePromptRequest::new(LLMObsUpdatePromptData::new(
        LLMObsUpdatePromptDataAttributes::new(),
        LLMObsPromptType::PROMPT_TEMPLATES,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdateLLMObsPrompt", true);
    let api = AgentObservabilityAPI::with_config(configuration);
    let resp = api
        .update_llm_obs_prompt("prompt_id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
