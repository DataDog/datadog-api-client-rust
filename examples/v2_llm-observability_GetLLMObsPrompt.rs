// Get an LLM Observability prompt returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_llm_observability::GetLLMObsPromptOptionalParams;
use datadog_api_client::datadogV2::api_llm_observability::LLMObservabilityAPI;

#[tokio::main]
async fn main() {
    // there is a valid "prompt" in the system
    let prompt_data_attributes_prompt_id =
        std::env::var("PROMPT_DATA_ATTRIBUTES_PROMPT_ID").unwrap();
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.GetLLMObsPrompt", true);
    let api = LLMObservabilityAPI::with_config(configuration);
    let resp = api
        .get_llm_obs_prompt(
            prompt_data_attributes_prompt_id.clone(),
            GetLLMObsPromptOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
