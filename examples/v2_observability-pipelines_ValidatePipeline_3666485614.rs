// Validate an observability pipeline with spike protection processor returns "OK"
// response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_observability_pipelines::ObservabilityPipelinesAPI;
use datadog_api_client::datadogV2::model::ObservabilityPipelineConfig;
use datadog_api_client::datadogV2::model::ObservabilityPipelineConfigDestinationItem;
use datadog_api_client::datadogV2::model::ObservabilityPipelineConfigProcessorGroup;
use datadog_api_client::datadogV2::model::ObservabilityPipelineConfigProcessorItem;
use datadog_api_client::datadogV2::model::ObservabilityPipelineConfigSourceItem;
use datadog_api_client::datadogV2::model::ObservabilityPipelineDataAttributes;
use datadog_api_client::datadogV2::model::ObservabilityPipelineDatadogAgentSource;
use datadog_api_client::datadogV2::model::ObservabilityPipelineDatadogAgentSourceType;
use datadog_api_client::datadogV2::model::ObservabilityPipelineDatadogLogsDestination;
use datadog_api_client::datadogV2::model::ObservabilityPipelineDatadogLogsDestinationType;
use datadog_api_client::datadogV2::model::ObservabilityPipelineSpec;
use datadog_api_client::datadogV2::model::ObservabilityPipelineSpecData;
use datadog_api_client::datadogV2::model::ObservabilityPipelineSpikeProtectionProcessor;
use datadog_api_client::datadogV2::model::ObservabilityPipelineSpikeProtectionProcessorSpikeAction;
use datadog_api_client::datadogV2::model::ObservabilityPipelineSpikeProtectionProcessorType;

#[tokio::main]
async fn main() {
    let body =
        ObservabilityPipelineSpec::new(
            ObservabilityPipelineSpecData::new(
                ObservabilityPipelineDataAttributes::new(
                    ObservabilityPipelineConfig::new(
                        vec![
                            ObservabilityPipelineConfigDestinationItem::ObservabilityPipelineDatadogLogsDestination(
                                Box::new(
                                    ObservabilityPipelineDatadogLogsDestination::new(
                                        "datadog-logs-destination".to_string(),
                                        vec!["my-processor-group".to_string()],
                                        ObservabilityPipelineDatadogLogsDestinationType::DATADOG_LOGS,
                                    ),
                                ),
                            )
                        ],
                        vec![
                            ObservabilityPipelineConfigSourceItem::ObservabilityPipelineDatadogAgentSource(
                                Box::new(
                                    ObservabilityPipelineDatadogAgentSource::new(
                                        "datadog-agent-source".to_string(),
                                        ObservabilityPipelineDatadogAgentSourceType::DATADOG_AGENT,
                                    ),
                                ),
                            )
                        ],
                    ).processor_groups(
                        vec![
                            ObservabilityPipelineConfigProcessorGroup::new(
                                true,
                                "my-processor-group".to_string(),
                                "service:my-service".to_string(),
                                vec!["datadog-agent-source".to_string()],
                                vec![
                                    ObservabilityPipelineConfigProcessorItem
                                    ::ObservabilityPipelineSpikeProtectionProcessor(
                                        Box::new(
                                            ObservabilityPipelineSpikeProtectionProcessor::new(
                                                true,
                                                "spike-protection-processor".to_string(),
                                                "*".to_string(),
                                                "MySpikeProtection".to_string(),
                                                ObservabilityPipelineSpikeProtectionProcessorSpikeAction::DROP,
                                                ObservabilityPipelineSpikeProtectionProcessorType::SPIKE_PROTECTION,
                                            )
                                                .minimal_threshold(107374182400)
                                                .spike_threshold(5.0 as f64),
                                        ),
                                    )
                                ],
                            )
                        ],
                    ),
                    "Pipeline with Spike Protection".to_string(),
                ),
                "pipelines".to_string(),
            ),
        );
    let configuration = datadog::Configuration::new();
    let api = ObservabilityPipelinesAPI::with_config(configuration);
    let resp = api.validate_pipeline(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
