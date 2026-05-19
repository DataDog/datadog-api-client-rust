// Search LLM Observability experimentation entities returns "Partial Content —
// more results are available. Use `meta.after` as the next `page.cursor`."
// response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_llm_observability::LLMObservabilityAPI;
use datadog_api_client::datadogV2::model::LLMObsExperimentationContentPreview;
use datadog_api_client::datadogV2::model::LLMObsExperimentationCursorPage;
use datadog_api_client::datadogV2::model::LLMObsExperimentationFilter;
use datadog_api_client::datadogV2::model::LLMObsExperimentationInclude;
use datadog_api_client::datadogV2::model::LLMObsExperimentationSearchDataAttributesRequest;
use datadog_api_client::datadogV2::model::LLMObsExperimentationSearchDataRequest;
use datadog_api_client::datadogV2::model::LLMObsExperimentationSearchRequest;
use datadog_api_client::datadogV2::model::LLMObsExperimentationType;

#[tokio::main]
async fn main() {
    let body =
        LLMObsExperimentationSearchRequest::new(LLMObsExperimentationSearchDataRequest::new(
            LLMObsExperimentationSearchDataAttributesRequest::new(
                LLMObsExperimentationFilter::new(vec!["experiments".to_string()])
                    .include_deleted(false)
                    .is_deleted(false)
                    .query("my experiment".to_string())
                    .version(None),
            )
            .content_preview(LLMObsExperimentationContentPreview::new().limit(500))
            .include(LLMObsExperimentationInclude::new().user_data(false))
            .page(LLMObsExperimentationCursorPage::new().limit(100)),
            LLMObsExperimentationType::EXPERIMENTATION,
        ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.SearchLLMObsExperimentation", true);
    let api = LLMObservabilityAPI::with_config(configuration);
    let resp = api.search_llm_obs_experimentation(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
