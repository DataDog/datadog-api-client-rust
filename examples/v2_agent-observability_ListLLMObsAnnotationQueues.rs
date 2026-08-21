// List Agent Observability annotation queues returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_agent_observability::AgentObservabilityAPI;
use datadog_api_client::datadogV2::api_agent_observability::ListLLMObsAnnotationQueuesOptionalParams;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ListLLMObsAnnotationQueues", true);
    let api = AgentObservabilityAPI::with_config(configuration);
    let resp = api
        .list_llm_obs_annotation_queues(ListLLMObsAnnotationQueuesOptionalParams::default())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
