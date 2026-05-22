// Get annotated interactions by content IDs returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_llm_observability::GetLLMObsAnnotatedInteractionsByTraceIDsOptionalParams;
use datadog_api_client::datadogV2::api_llm_observability::LLMObservabilityAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration
        .set_unstable_operation_enabled("v2.GetLLMObsAnnotatedInteractionsByTraceIDs", true);
    let api = LLMObservabilityAPI::with_config(configuration);
    let resp = api
        .get_llm_obs_annotated_interactions_by_trace_i_ds(
            vec![],
            GetLLMObsAnnotatedInteractionsByTraceIDsOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
