// Update an LLM Observability experiment returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_llm_observability::LLMObservabilityAPI;
use datadog_api_client::datadogV2::model::LLMObsExperimentStatus;
use datadog_api_client::datadogV2::model::LLMObsExperimentType;
use datadog_api_client::datadogV2::model::LLMObsExperimentUpdateDataAttributesRequest;
use datadog_api_client::datadogV2::model::LLMObsExperimentUpdateDataRequest;
use datadog_api_client::datadogV2::model::LLMObsExperimentUpdateRequest;

#[tokio::main]
async fn main() {
    let body = LLMObsExperimentUpdateRequest::new(LLMObsExperimentUpdateDataRequest::new(
        LLMObsExperimentUpdateDataAttributesRequest::new()
            .dataset_id("9f64e5c7-dc5a-45c8-a17c-1b85f0bec97d".to_string())
            .status(LLMObsExperimentStatus::COMPLETED),
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
