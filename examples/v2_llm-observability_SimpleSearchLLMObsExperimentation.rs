// Simple search experimentation entities returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_llm_observability::LLMObservabilityAPI;
use datadog_api_client::datadogV2::model::LLMObsExperimentationContentPreview;
use datadog_api_client::datadogV2::model::LLMObsExperimentationFilter;
use datadog_api_client::datadogV2::model::LLMObsExperimentationInclude;
use datadog_api_client::datadogV2::model::LLMObsExperimentationNumberPage;
use datadog_api_client::datadogV2::model::LLMObsExperimentationSimpleSearchDataAttributesRequest;
use datadog_api_client::datadogV2::model::LLMObsExperimentationSimpleSearchDataRequest;
use datadog_api_client::datadogV2::model::LLMObsExperimentationSimpleSearchRequest;
use datadog_api_client::datadogV2::model::LLMObsExperimentationSortField;
use datadog_api_client::datadogV2::model::LLMObsExperimentationSortFieldDirection;
use datadog_api_client::datadogV2::model::LLMObsExperimentationType;

#[tokio::main]
async fn main() {
    let body = LLMObsExperimentationSimpleSearchRequest::new(
        LLMObsExperimentationSimpleSearchDataRequest::new(
            LLMObsExperimentationSimpleSearchDataAttributesRequest::new(
                LLMObsExperimentationFilter::new(vec!["experiments".to_string()])
                    .include_deleted(false)
                    .is_deleted(false)
                    .query("my experiment".to_string())
                    .version(None),
            )
            .content_preview(LLMObsExperimentationContentPreview::new().limit(500))
            .include(LLMObsExperimentationInclude::new().user_data(false))
            .page(LLMObsExperimentationNumberPage::new().limit(50).number(1))
            .sort(vec![LLMObsExperimentationSortField::new(
                "created_at".to_string(),
            )
            .direction(LLMObsExperimentationSortFieldDirection::DESC)]),
            LLMObsExperimentationType::EXPERIMENTATION,
        ),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.SimpleSearchLLMObsExperimentation", true);
    let api = LLMObservabilityAPI::with_config(configuration);
    let resp = api.simple_search_llm_obs_experimentation(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
