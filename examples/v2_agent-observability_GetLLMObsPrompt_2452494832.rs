// Get an Agent Observability prompt by environment returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_agent_observability::AgentObservabilityAPI;
use datadog_api_client::datadogV2::api_agent_observability::GetLLMObsPromptOptionalParams;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.GetLLMObsPrompt", true);
    let api = AgentObservabilityAPI::with_config(configuration);
    let resp = api
        .get_llm_obs_prompt(
            "prompt_id".to_string(),
            GetLLMObsPromptOptionalParams::default().environment("production".to_string()),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
