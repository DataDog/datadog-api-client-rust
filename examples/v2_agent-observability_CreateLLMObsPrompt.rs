// Create an Agent Observability prompt returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_agent_observability::AgentObservabilityAPI;
use datadog_api_client::datadogV2::model::LLMObsCreatePromptData;
use datadog_api_client::datadogV2::model::LLMObsCreatePromptDataAttributes;
use datadog_api_client::datadogV2::model::LLMObsCreatePromptRequest;
use datadog_api_client::datadogV2::model::LLMObsPromptTemplate;
use datadog_api_client::datadogV2::model::LLMObsPromptType;
use datadog_api_client::datadogV2::model::LLMObsPromptVersionLabel;

#[tokio::main]
async fn main() {
    let body = LLMObsCreatePromptRequest::new(LLMObsCreatePromptData::new(
        LLMObsCreatePromptDataAttributes::new(
            "customer-support-assistant".to_string(),
            LLMObsPromptTemplate::LLMObsPromptTextTemplate(
                "You are a helpful assistant for .".to_string(),
            ),
        )
        .env_ids(vec![])
        .labels(vec![LLMObsPromptVersionLabel::PRODUCTION]),
        LLMObsPromptType::PROMPT_TEMPLATES,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateLLMObsPrompt", true);
    let api = AgentObservabilityAPI::with_config(configuration);
    let resp = api.create_llm_obs_prompt(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
