// Update a pipeline returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_observability_pipelines::ObservabilityPipelinesAPI;
use datadog_api_client::datadogV2::model::Pipeline;
use datadog_api_client::datadogV2::model::PipelineConfig;
use datadog_api_client::datadogV2::model::PipelineConfigDestination;
use datadog_api_client::datadogV2::model::PipelineConfigProcessor;
use datadog_api_client::datadogV2::model::PipelineConfigSource;
use datadog_api_client::datadogV2::model::PipelineData;
use datadog_api_client::datadogV2::model::PipelineDataAttributes;
use datadog_api_client::datadogV2::model::PipelineDatadogAgentSource;
use datadog_api_client::datadogV2::model::PipelineDatadogAgentSourceType;
use datadog_api_client::datadogV2::model::PipelineDatadogLogsDestination;
use datadog_api_client::datadogV2::model::PipelineDatadogLogsDestinationType;
use datadog_api_client::datadogV2::model::PipelineFilterProcessor;
use datadog_api_client::datadogV2::model::PipelineFilterProcessorType;

#[tokio::main]
async fn main() {
    // there is a valid "pipeline" in the system
    let pipeline_data_id = std::env::var("PIPELINE_DATA_ID").unwrap();
    let body = Pipeline::new(PipelineData::new(
        PipelineDataAttributes::new(
            PipelineConfig::new(
                vec![PipelineConfigDestination::PipelineDatadogLogsDestination(
                    Box::new(PipelineDatadogLogsDestination::new(
                        "updated-datadog-logs-destination-id".to_string(),
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
            "Updated Pipeline Name".to_string(),
        ),
        pipeline_data_id.clone(),
        "pipelines".to_string(),
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdatePipeline", true);
    let api = ObservabilityPipelinesAPI::with_config(configuration);
    let resp = api.update_pipeline(pipeline_data_id.clone(), body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
