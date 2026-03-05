// Create an LLM Observability experiment returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_llm_observability::LLMObservabilityAPI;
use datadog_api_client::datadogV2::model::LLMObsExperimentDataAttributesRequest;
use datadog_api_client::datadogV2::model::LLMObsExperimentDataRequest;
use datadog_api_client::datadogV2::model::LLMObsExperimentRequest;
use datadog_api_client::datadogV2::model::LLMObsExperimentType;

#[tokio::main]
async fn main() {
    let body = LLMObsExperimentRequest::new(LLMObsExperimentDataRequest::new(
        LLMObsExperimentDataAttributesRequest::new(
            "9f64e5c7-dc5a-45c8-a17c-1b85f0bec97d".to_string(),
            "My Experiment v1".to_string(),
            "a33671aa-24fd-4dcd-9b33-a8ec7dde7751".to_string(),
        ),
        LLMObsExperimentType::EXPERIMENTS,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateLLMObsExperiment", true);
    let api = LLMObservabilityAPI::with_config(configuration);
    let resp = api.create_llm_obs_experiment(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
