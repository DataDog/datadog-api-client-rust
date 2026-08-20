// Update an Agent Observability prompt version returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_agent_observability::AgentObservabilityAPI;
use datadog_api_client::datadogV2::model::LLMObsPromptVersionLabel;
use datadog_api_client::datadogV2::model::LLMObsPromptVersionType;
use datadog_api_client::datadogV2::model::LLMObsUpdatePromptVersionData;
use datadog_api_client::datadogV2::model::LLMObsUpdatePromptVersionDataAttributes;
use datadog_api_client::datadogV2::model::LLMObsUpdatePromptVersionRequest;

#[tokio::main]
async fn main() {
    let body = LLMObsUpdatePromptVersionRequest::new(LLMObsUpdatePromptVersionData::new(
        LLMObsUpdatePromptVersionDataAttributes::new()
            .env_ids(vec![])
            .labels(vec![LLMObsPromptVersionLabel::PRODUCTION]),
        LLMObsPromptVersionType::PROMPT_TEMPLATE_VERSIONS,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdateLLMObsPromptVersion", true);
    let api = AgentObservabilityAPI::with_config(configuration);
    let resp = api
        .update_llm_obs_prompt_version("prompt_id".to_string(), 9223372036854775807, body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
