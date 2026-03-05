// Update an LLM Observability experiment returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_llm_observability::LLMObservabilityAPI;
use datadog_api_client::datadogV2::model::LLMObsExperimentType;
use datadog_api_client::datadogV2::model::LLMObsExperimentUpdateDataAttributesRequest;
use datadog_api_client::datadogV2::model::LLMObsExperimentUpdateDataRequest;
use datadog_api_client::datadogV2::model::LLMObsExperimentUpdateRequest;

#[tokio::main]
async fn main() {
    let body = LLMObsExperimentUpdateRequest::new(LLMObsExperimentUpdateDataRequest::new(
        LLMObsExperimentUpdateDataAttributesRequest::new(),
        LLMObsExperimentType::EXPERIMENTS,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdateLLMObsExperiment", true);
    let api = LLMObservabilityAPI::with_config(configuration);
    let resp = api
        .update_llm_obs_experiment("experiment_id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
