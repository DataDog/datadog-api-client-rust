// Validate an observability pipeline with Splunk HEC destination token_strategy
// returns "OK" response
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
use datadog_api_client::datadogV2::model::ObservabilityPipelineFilterProcessor;
use datadog_api_client::datadogV2::model::ObservabilityPipelineFilterProcessorType;
use datadog_api_client::datadogV2::model::ObservabilityPipelineSpec;
use datadog_api_client::datadogV2::model::ObservabilityPipelineSpecData;
use datadog_api_client::datadogV2::model::ObservabilityPipelineSplunkHecDestination;
use datadog_api_client::datadogV2::model::ObservabilityPipelineSplunkHecDestinationTokenStrategy;
use datadog_api_client::datadogV2::model::ObservabilityPipelineSplunkHecDestinationType;

#[tokio::main]
async fn main() {
    let body =
        ObservabilityPipelineSpec::new(
            ObservabilityPipelineSpecData::new(
                ObservabilityPipelineDataAttributes::new(
                    ObservabilityPipelineConfig::new(
                        vec![
                            ObservabilityPipelineConfigDestinationItem::ObservabilityPipelineSplunkHecDestination(
                                Box::new(
                                    ObservabilityPipelineSplunkHecDestination::new(
                                        "splunk-hec-destination".to_string(),
                                        vec!["my-processor-group".to_string()],
                                        ObservabilityPipelineSplunkHecDestinationType::SPLUNK_HEC,
                                    )
                                        .token_key("SPLUNK_HEC_TOKEN".to_string())
                                        .token_strategy(
                                            ObservabilityPipelineSplunkHecDestinationTokenStrategy::CUSTOM,
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
                                    ObservabilityPipelineConfigProcessorItem::ObservabilityPipelineFilterProcessor(
                                        Box::new(
                                            ObservabilityPipelineFilterProcessor::new(
                                                true,
                                                "filter-processor".to_string(),
                                                "status:error".to_string(),
                                                ObservabilityPipelineFilterProcessorType::FILTER,
                                            ),
                                        ),
                                    )
                                ],
                            )
                        ],
                    ),
                    "Pipeline with Splunk HEC token_strategy".to_string(),
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
