// Delete LLM Observability projects returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_llm_observability::LLMObservabilityAPI;
use datadog_api_client::datadogV2::model::LLMObsDeleteProjectsDataAttributesRequest;
use datadog_api_client::datadogV2::model::LLMObsDeleteProjectsDataRequest;
use datadog_api_client::datadogV2::model::LLMObsDeleteProjectsRequest;
use datadog_api_client::datadogV2::model::LLMObsProjectType;

#[tokio::main]
async fn main() {
    let body = LLMObsDeleteProjectsRequest::new(LLMObsDeleteProjectsDataRequest::new(
        LLMObsDeleteProjectsDataAttributesRequest::new(vec![
            "a33671aa-24fd-4dcd-9b33-a8ec7dde7751".to_string(),
        ]),
        LLMObsProjectType::PROJECTS,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.DeleteLLMObsProjects", true);
    let api = LLMObservabilityAPI::with_config(configuration);
    let resp = api.delete_llm_obs_projects(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
