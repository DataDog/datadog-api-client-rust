// Validate an observability pipeline with spike protection processor threshold
// overrides returns "OK" response
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
use datadog_api_client::datadogV2::model::ObservabilityPipelineFieldValue;
use datadog_api_client::datadogV2::model::ObservabilityPipelineSpec;
use datadog_api_client::datadogV2::model::ObservabilityPipelineSpecData;
use datadog_api_client::datadogV2::model::ObservabilityPipelineSpikeProtectionProcessor;
use datadog_api_client::datadogV2::model::ObservabilityPipelineSpikeProtectionProcessorActionWhenMissingPartitions;
use datadog_api_client::datadogV2::model::ObservabilityPipelineSpikeProtectionProcessorMinimalThreshold;
use datadog_api_client::datadogV2::model::ObservabilityPipelineSpikeProtectionProcessorMinimalThresholdAlwaysKeep;
use datadog_api_client::datadogV2::model::ObservabilityPipelineSpikeProtectionProcessorMinimalThresholdAlwaysKeepType;
use datadog_api_client::datadogV2::model::ObservabilityPipelineSpikeProtectionProcessorMinimalThresholdBytes;
use datadog_api_client::datadogV2::model::ObservabilityPipelineSpikeProtectionProcessorMinimalThresholdBytesType;
use datadog_api_client::datadogV2::model::ObservabilityPipelineSpikeProtectionProcessorSpikeAction;
use datadog_api_client::datadogV2::model::ObservabilityPipelineSpikeProtectionProcessorThresholdOverride;
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
                                                "MyPartitionedSpikeProtection".to_string(),
                                                ObservabilityPipelineSpikeProtectionProcessorSpikeAction::NO_ACTION,
                                                ObservabilityPipelineSpikeProtectionProcessorType::SPIKE_PROTECTION,
                                            )
                                                .action_when_missing_partitions(
                                                    ObservabilityPipelineSpikeProtectionProcessorActionWhenMissingPartitions
                                                    ::DEFAULT_BUCKET,
                                                )
                                                .minimal_threshold_overrides(
                                                    vec![
                                                        ObservabilityPipelineSpikeProtectionProcessorThresholdOverride
                                                        ::new(
                                                            vec![
                                                                ObservabilityPipelineFieldValue::new(
                                                                    "service".to_string(),
                                                                    "checkout".to_string(),
                                                                )
                                                            ],
                                                            ObservabilityPipelineSpikeProtectionProcessorMinimalThreshold
                                                            ::ObservabilityPipelineSpikeProtectionProcessorMinimalThresholdBytes(
                                                                Box::new(
                                                                    ObservabilityPipelineSpikeProtectionProcessorMinimalThresholdBytes
                                                                    ::new(
                                                                        53687091200,
                                                                        ObservabilityPipelineSpikeProtectionProcessorMinimalThresholdBytesType
                                                                        ::THRESHOLD,
                                                                    ),
                                                                ),
                                                            ),
                                                        ),
                                                        ObservabilityPipelineSpikeProtectionProcessorThresholdOverride
                                                        ::new(
                                                            vec![
                                                                ObservabilityPipelineFieldValue::new(
                                                                    "service".to_string(),
                                                                    "payments".to_string(),
                                                                )
                                                            ],
                                                            ObservabilityPipelineSpikeProtectionProcessorMinimalThreshold
                                                            ::ObservabilityPipelineSpikeProtectionProcessorMinimalThresholdAlwaysKeep(
                                                                Box::new(
                                                                    ObservabilityPipelineSpikeProtectionProcessorMinimalThresholdAlwaysKeep
                                                                    ::new(
                                                                        ObservabilityPipelineSpikeProtectionProcessorMinimalThresholdAlwaysKeepType
                                                                        ::ALWAYS_KEEP,
                                                                    ),
                                                                ),
                                                            ),
                                                        )
                                                    ],
                                                )
                                                .partition_fields(vec!["service".to_string()]),
                                        ),
                                    )
                                ],
                            )
                        ],
                    ),
                    "Pipeline with Spike Protection Threshold Overrides".to_string(),
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
