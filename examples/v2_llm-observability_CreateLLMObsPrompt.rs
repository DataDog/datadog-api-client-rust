// Create an LLM Observability prompt returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_llm_observability::LLMObservabilityAPI;
use datadog_api_client::datadogV2::model::LLMObsCreatePromptData;
use datadog_api_client::datadogV2::model::LLMObsCreatePromptDataAttributes;
use datadog_api_client::datadogV2::model::LLMObsCreatePromptRequest;
use datadog_api_client::datadogV2::model::LLMObsPromptChatMessage;
use datadog_api_client::datadogV2::model::LLMObsPromptTemplate;
use datadog_api_client::datadogV2::model::LLMObsPromptType;

#[tokio::main]
async fn main() {
    let body = LLMObsCreatePromptRequest::new(LLMObsCreatePromptData::new(
        LLMObsCreatePromptDataAttributes::new(
            "Example-LLM-Observability".to_string(),
            LLMObsPromptTemplate::LLMObsPromptChatTemplate(vec![
                LLMObsPromptChatMessage::new(
                    "You are a helpful customer support assistant for {{company_name}}."
                        .to_string(),
                    "system".to_string(),
                ),
                LLMObsPromptChatMessage::new(
                    "Help {{customer_name}} with this question: {{question}}".to_string(),
                    "user".to_string(),
                ),
            ]),
        )
        .title("Customer Support Assistant".to_string()),
        LLMObsPromptType::PROMPT_TEMPLATES,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateLLMObsPrompt", true);
    let api = LLMObservabilityAPI::with_config(configuration);
    let resp = api.create_llm_obs_prompt(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
