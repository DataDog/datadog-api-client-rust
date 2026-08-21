// Create a new Agent Observability prompt version returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_agent_observability::AgentObservabilityAPI;
use datadog_api_client::datadogV2::model::LLMObsCreatePromptVersionData;
use datadog_api_client::datadogV2::model::LLMObsCreatePromptVersionDataAttributes;
use datadog_api_client::datadogV2::model::LLMObsCreatePromptVersionRequest;
use datadog_api_client::datadogV2::model::LLMObsPromptTemplate;
use datadog_api_client::datadogV2::model::LLMObsPromptVersionLabel;
use datadog_api_client::datadogV2::model::LLMObsPromptVersionType;

#[tokio::main]
async fn main() {
    let body = LLMObsCreatePromptVersionRequest::new(LLMObsCreatePromptVersionData::new(
        LLMObsCreatePromptVersionDataAttributes::new(
            LLMObsPromptTemplate::LLMObsPromptTextTemplate(
                "You are a helpful assistant for .".to_string(),
            ),
        )
        .env_ids(vec![])
        .labels(vec![LLMObsPromptVersionLabel::PRODUCTION]),
        LLMObsPromptVersionType::PROMPT_TEMPLATE_VERSIONS,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateLLMObsPromptVersion", true);
    let api = AgentObservabilityAPI::with_config(configuration);
    let resp = api
        .create_llm_obs_prompt_version("prompt_id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
