// Push events for an LLM Observability experiment returns "Accepted" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_llm_observability::LLMObservabilityAPI;
use datadog_api_client::datadogV2::model::LLMObsEventType;
use datadog_api_client::datadogV2::model::LLMObsExperimentEventsDataAttributesRequest;
use datadog_api_client::datadogV2::model::LLMObsExperimentEventsDataRequest;
use datadog_api_client::datadogV2::model::LLMObsExperimentEventsRequest;
use datadog_api_client::datadogV2::model::LLMObsExperimentMetric;
use datadog_api_client::datadogV2::model::LLMObsExperimentMetricError;
use datadog_api_client::datadogV2::model::LLMObsExperimentSpan;
use datadog_api_client::datadogV2::model::LLMObsExperimentSpanError;
use datadog_api_client::datadogV2::model::LLMObsExperimentSpanMeta;
use datadog_api_client::datadogV2::model::LLMObsMetricAssessment;
use datadog_api_client::datadogV2::model::LLMObsMetricScoreType;

#[tokio::main]
async fn main() {
    let body = LLMObsExperimentEventsRequest::new(LLMObsExperimentEventsDataRequest::new(
        LLMObsExperimentEventsDataAttributesRequest::new()
            .metrics(vec![LLMObsExperimentMetric::new(
                "faithfulness".to_string(),
                LLMObsMetricScoreType::SCORE,
                "span-7a1b2c3d".to_string(),
                1705314600000,
            )
            .assessment(LLMObsMetricAssessment::PASS)
            .error(LLMObsExperimentMetricError::new())
            .tags(vec![])])
            .spans(vec![LLMObsExperimentSpan::new(
                "9f64e5c7-dc5a-45c8-a17c-1b85f0bec97d".to_string(),
                1500000000,
                "llm_call".to_string(),
                "a33671aa-24fd-4dcd-9b33-a8ec7dde7751".to_string(),
                "span-7a1b2c3d".to_string(),
                1705314600000000000,
                "ok".to_string(),
                "abc123def456".to_string(),
            )
            .meta(
                LLMObsExperimentSpanMeta::new()
                    .error(LLMObsExperimentSpanError::new())
                    .input(None)
                    .output(None),
            )
            .tags(vec![])]),
        LLMObsEventType::EVENTS,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateLLMObsExperimentEvents", true);
    let api = LLMObservabilityAPI::with_config(configuration);
    let resp = api
        .create_llm_obs_experiment_events("experiment_id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
