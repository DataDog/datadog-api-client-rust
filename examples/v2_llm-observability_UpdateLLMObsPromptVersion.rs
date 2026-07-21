// Update a specific LLM Observability prompt version returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_llm_observability::LLMObservabilityAPI;
use datadog_api_client::datadogV2::model::LLMObsPromptVersionType;
use datadog_api_client::datadogV2::model::LLMObsUpdatePromptVersionData;
use datadog_api_client::datadogV2::model::LLMObsUpdatePromptVersionDataAttributes;
use datadog_api_client::datadogV2::model::LLMObsUpdatePromptVersionRequest;

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
    let body = LLMObsUpdatePromptVersionRequest::new(LLMObsUpdatePromptVersionData::new(
        LLMObsUpdatePromptVersionDataAttributes::new().description(
            "Give concise answers and cite relevant help-center articles.".to_string(),
        ),
        LLMObsPromptVersionType::PROMPT_TEMPLATE_VERSIONS,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdateLLMObsPromptVersion", true);
    let api = LLMObservabilityAPI::with_config(configuration);
    let resp = api
        .update_llm_obs_prompt_version(
            prompt_data_attributes_prompt_id.clone(),
            prompt_version_data_attributes_version.clone(),
            body,
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
