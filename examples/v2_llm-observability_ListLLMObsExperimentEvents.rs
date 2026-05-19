// List events for an LLM Observability experiment returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_llm_observability::LLMObservabilityAPI;
use datadog_api_client::datadogV2::api_llm_observability::ListLLMObsExperimentEventsOptionalParams;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ListLLMObsExperimentEvents", true);
    let api = LLMObservabilityAPI::with_config(configuration);
    let resp = api
        .list_llm_obs_experiment_events(
            "experiment_id".to_string(),
            ListLLMObsExperimentEventsOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
