// Create a new pipeline returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_observability_pipelines::ObservabilityPipelinesAPI;
use datadog_api_client::datadogV2::model::PipelineConfig;
use datadog_api_client::datadogV2::model::PipelineConfigDestination;
use datadog_api_client::datadogV2::model::PipelineConfigProcessor;
use datadog_api_client::datadogV2::model::PipelineConfigSource;
use datadog_api_client::datadogV2::model::PipelineCreateRequest;
use datadog_api_client::datadogV2::model::PipelineCreateRequestData;
use datadog_api_client::datadogV2::model::PipelineDataAttributes;
use datadog_api_client::datadogV2::model::PipelineDatadogAgentSource;
use datadog_api_client::datadogV2::model::PipelineDatadogAgentSourceType;
use datadog_api_client::datadogV2::model::PipelineDatadogLogsDestination;
use datadog_api_client::datadogV2::model::PipelineDatadogLogsDestinationType;
use datadog_api_client::datadogV2::model::PipelineFilterProcessor;
use datadog_api_client::datadogV2::model::PipelineFilterProcessorType;

#[tokio::main]
async fn main() {
    let body = PipelineCreateRequest::new(PipelineCreateRequestData::new(
        PipelineDataAttributes::new(
            PipelineConfig::new(
                vec![PipelineConfigDestination::PipelineDatadogLogsDestination(
                    Box::new(PipelineDatadogLogsDestination::new(
                        "datadog-logs-destination".to_string(),
                        vec!["filter-processor".to_string()],
                        PipelineDatadogLogsDestinationType::DATADOG_LOGS,
                    )),
                )],
                vec![PipelineConfigProcessor::PipelineFilterProcessor(Box::new(
                    PipelineFilterProcessor::new(
                        "filter-processor".to_string(),
                        "service:my-service".to_string(),
                        vec!["datadog-agent-source".to_string()],
                        PipelineFilterProcessorType::FILTER,
                    ),
                ))],
                vec![PipelineConfigSource::PipelineDatadogAgentSource(Box::new(
                    PipelineDatadogAgentSource::new(
                        "datadog-agent-source".to_string(),
                        PipelineDatadogAgentSourceType::DATADOG_AGENT,
                    ),
                ))],
            ),
            "Main Observability Pipeline".to_string(),
        ),
        "pipelines".to_string(),
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreatePipeline", true);
    let api = ObservabilityPipelinesAPI::with_config(configuration);
    let resp = api.create_pipeline(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
