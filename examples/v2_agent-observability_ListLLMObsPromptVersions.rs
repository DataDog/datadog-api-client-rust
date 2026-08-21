// List versions of an Agent Observability prompt returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_agent_observability::AgentObservabilityAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ListLLMObsPromptVersions", true);
    let api = AgentObservabilityAPI::with_config(configuration);
    let resp = api
        .list_llm_obs_prompt_versions("prompt_id".to_string())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
