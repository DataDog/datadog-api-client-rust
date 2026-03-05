// Delete LLM Observability experiments returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_llm_observability::LLMObservabilityAPI;
use datadog_api_client::datadogV2::model::LLMObsDeleteExperimentsDataAttributesRequest;
use datadog_api_client::datadogV2::model::LLMObsDeleteExperimentsDataRequest;
use datadog_api_client::datadogV2::model::LLMObsDeleteExperimentsRequest;
use datadog_api_client::datadogV2::model::LLMObsExperimentType;

#[tokio::main]
async fn main() {
    let body = LLMObsDeleteExperimentsRequest::new(LLMObsDeleteExperimentsDataRequest::new(
        LLMObsDeleteExperimentsDataAttributesRequest::new(vec![
            "3fd6b5e0-8910-4b1c-a7d0-5b84de329012".to_string(),
        ]),
        LLMObsExperimentType::EXPERIMENTS,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.DeleteLLMObsExperiments", true);
    let api = LLMObservabilityAPI::with_config(configuration);
    let resp = api.delete_llm_obs_experiments(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
