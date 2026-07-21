// Create a new LLM Observability prompt version returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_llm_observability::LLMObservabilityAPI;
use datadog_api_client::datadogV2::model::LLMObsCreatePromptVersionData;
use datadog_api_client::datadogV2::model::LLMObsCreatePromptVersionDataAttributes;
use datadog_api_client::datadogV2::model::LLMObsCreatePromptVersionRequest;
use datadog_api_client::datadogV2::model::LLMObsPromptChatMessage;
use datadog_api_client::datadogV2::model::LLMObsPromptTemplate;
use datadog_api_client::datadogV2::model::LLMObsPromptVersionType;

#[tokio::main]
async fn main() {
    // there is a valid "prompt" in the system
    let prompt_data_attributes_prompt_id =
        std::env::var("PROMPT_DATA_ATTRIBUTES_PROMPT_ID").unwrap();
    let body = LLMObsCreatePromptVersionRequest::new(LLMObsCreatePromptVersionData::new(
        LLMObsCreatePromptVersionDataAttributes::new(
            LLMObsPromptTemplate::LLMObsPromptChatTemplate(vec![
                LLMObsPromptChatMessage::new(
                    "You are a concise customer support assistant for {{company_name}}."
                        .to_string(),
                    "system".to_string(),
                ),
                LLMObsPromptChatMessage::new(
                    "Answer {{customer_name}}'s question: {{question}}".to_string(),
                    "user".to_string(),
                ),
            ]),
        ),
        LLMObsPromptVersionType::PROMPT_TEMPLATE_VERSIONS,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateLLMObsPromptVersion", true);
    let api = LLMObservabilityAPI::with_config(configuration);
    let resp = api
        .create_llm_obs_prompt_version(prompt_data_attributes_prompt_id.clone(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
