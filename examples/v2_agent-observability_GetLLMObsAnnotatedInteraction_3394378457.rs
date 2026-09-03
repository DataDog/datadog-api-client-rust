// Get an annotated queue interaction returns "OK" response with pagination
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_agent_observability::AgentObservabilityAPI;
use datadog_api_client::datadogV2::api_agent_observability::GetLLMObsAnnotatedInteractionOptionalParams;
use futures_util::pin_mut;
use futures_util::stream::StreamExt;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.GetLLMObsAnnotatedInteraction", true);
    let api = AgentObservabilityAPI::with_config(configuration);
    let response = api.get_llm_obs_annotated_interaction_with_pagination(
        "queue_id".to_string(),
        "interaction_id".to_string(),
        GetLLMObsAnnotatedInteractionOptionalParams::default(),
    );
    pin_mut!(response);
    while let Some(resp) = response.next().await {
        if let Ok(value) = resp {
            println!("{:#?}", value);
        } else {
            println!("{:#?}", resp.unwrap_err());
        }
    }
}
