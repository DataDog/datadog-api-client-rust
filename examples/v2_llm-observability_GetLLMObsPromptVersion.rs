// Get a specific LLM Observability prompt version returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_llm_observability::LLMObservabilityAPI;

#[tokio::main]
async fn main() {
    // there is a valid "prompt" in the system
    let prompt_data_attributes_prompt_id =
        std::env::var("PROMPT_DATA_ATTRIBUTES_PROMPT_ID").unwrap();

    // there is a valid "prompt_version" in the system
    let prompt_version_data_attributes_version: i64 =
        std::env::var("PROMPT_VERSION_DATA_ATTRIBUTES_VERSION")
            .unwrap()
            .parse()
            .unwrap();
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.GetLLMObsPromptVersion", true);
    let api = LLMObservabilityAPI::with_config(configuration);
    let resp = api
        .get_llm_obs_prompt_version(
            prompt_data_attributes_prompt_id.clone(),
            prompt_version_data_attributes_version.clone(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
