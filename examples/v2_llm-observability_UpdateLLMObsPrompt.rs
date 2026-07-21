// Update an LLM Observability prompt returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_llm_observability::LLMObservabilityAPI;
use datadog_api_client::datadogV2::model::LLMObsPromptType;
use datadog_api_client::datadogV2::model::LLMObsUpdatePromptData;
use datadog_api_client::datadogV2::model::LLMObsUpdatePromptDataAttributes;
use datadog_api_client::datadogV2::model::LLMObsUpdatePromptRequest;

#[tokio::main]
async fn main() {
    // there is a valid "prompt" in the system
    let prompt_data_attributes_prompt_id =
        std::env::var("PROMPT_DATA_ATTRIBUTES_PROMPT_ID").unwrap();
    let body = LLMObsUpdatePromptRequest::new(LLMObsUpdatePromptData::new(
        LLMObsUpdatePromptDataAttributes::new().title("Customer Support Assistant".to_string()),
        LLMObsPromptType::PROMPT_TEMPLATES,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdateLLMObsPrompt", true);
    let api = LLMObservabilityAPI::with_config(configuration);
    let resp = api
        .update_llm_obs_prompt(prompt_data_attributes_prompt_id.clone(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
