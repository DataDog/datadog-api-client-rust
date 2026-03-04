// Create an LLM Observability project returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_llm_observability::LLMObservabilityAPI;
use datadog_api_client::datadogV2::model::LLMObsProjectDataAttributesRequest;
use datadog_api_client::datadogV2::model::LLMObsProjectDataRequest;
use datadog_api_client::datadogV2::model::LLMObsProjectRequest;
use datadog_api_client::datadogV2::model::LLMObsProjectType;

#[tokio::main]
async fn main() {
    let body = LLMObsProjectRequest::new(LLMObsProjectDataRequest::new(
        LLMObsProjectDataAttributesRequest::new("My LLM Project".to_string()),
        LLMObsProjectType::PROJECTS,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateLLMObsProject", true);
    let api = LLMObservabilityAPI::with_config(configuration);
    let resp = api.create_llm_obs_project(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
