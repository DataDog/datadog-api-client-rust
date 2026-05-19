// Aggregate LLM Observability experimentation returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_llm_observability::LLMObservabilityAPI;
use datadog_api_client::datadogV2::model::LLMObsExperimentationAnalyticsAggregate;
use datadog_api_client::datadogV2::model::LLMObsExperimentationAnalyticsCompute;
use datadog_api_client::datadogV2::model::LLMObsExperimentationAnalyticsDataAttributesRequest;
use datadog_api_client::datadogV2::model::LLMObsExperimentationAnalyticsDataRequest;
use datadog_api_client::datadogV2::model::LLMObsExperimentationAnalyticsGroupBy;
use datadog_api_client::datadogV2::model::LLMObsExperimentationAnalyticsRequest;
use datadog_api_client::datadogV2::model::LLMObsExperimentationAnalyticsSearch;
use datadog_api_client::datadogV2::model::LLMObsExperimentationAnalyticsTimeRange;
use datadog_api_client::datadogV2::model::LLMObsExperimentationType;

#[tokio::main]
async fn main() {
    let body =
        LLMObsExperimentationAnalyticsRequest::new(LLMObsExperimentationAnalyticsDataRequest::new(
            LLMObsExperimentationAnalyticsDataAttributesRequest::new(
                LLMObsExperimentationAnalyticsAggregate::new(
                    vec![
                        LLMObsExperimentationAnalyticsCompute::new("score_value".to_string())
                            .name("avg_faithfulness".to_string()),
                    ],
                    vec!["experiment-evals".to_string()],
                    LLMObsExperimentationAnalyticsSearch::new(
                        "@experiment_id:3fd6b5e0-8910-4b1c-a7d0-5b84de329012".to_string(),
                    ),
                )
                .dataset_version(None)
                .group_by(vec![LLMObsExperimentationAnalyticsGroupBy::new(
                    "span_id".to_string(),
                )])
                .limit(Some(1000))
                .time(LLMObsExperimentationAnalyticsTimeRange::new(
                    1705312200000,
                    1705315800000,
                )),
            ),
            LLMObsExperimentationType::EXPERIMENTATION,
        ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.AggregateLLMObsExperimentation", true);
    let api = LLMObservabilityAPI::with_config(configuration);
    let resp = api.aggregate_llm_obs_experimentation(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
