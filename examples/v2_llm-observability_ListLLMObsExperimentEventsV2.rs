// List LLM Observability experiment events (v2) returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_llm_observability::LLMObservabilityAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ListLLMObsExperimentEventsV2", true);
    let api = LLMObservabilityAPI::with_config(configuration);
    let resp = api
        .list_llm_obs_experiment_events_v2("experiment_id".to_string())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
