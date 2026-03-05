// Update an LLM Observability project returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_llm_observability::LLMObservabilityAPI;
use datadog_api_client::datadogV2::model::LLMObsProjectType;
use datadog_api_client::datadogV2::model::LLMObsProjectUpdateDataAttributesRequest;
use datadog_api_client::datadogV2::model::LLMObsProjectUpdateDataRequest;
use datadog_api_client::datadogV2::model::LLMObsProjectUpdateRequest;

#[tokio::main]
async fn main() {
    let body = LLMObsProjectUpdateRequest::new(LLMObsProjectUpdateDataRequest::new(
        LLMObsProjectUpdateDataAttributesRequest::new(),
        LLMObsProjectType::PROJECTS,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdateLLMObsProject", true);
    let api = LLMObservabilityAPI::with_config(configuration);
    let resp = api
        .update_llm_obs_project("project_id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
