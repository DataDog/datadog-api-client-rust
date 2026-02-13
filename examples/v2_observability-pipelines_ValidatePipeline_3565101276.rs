// Validate an observability pipeline with OCSF mapper library mapping returns
// "OK" response
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
use datadog_api_client::datadogV2::model::ObservabilityPipelineOcsfMapperProcessor;
use datadog_api_client::datadogV2::model::ObservabilityPipelineOcsfMapperProcessorMapping;
use datadog_api_client::datadogV2::model::ObservabilityPipelineOcsfMapperProcessorMappingMapping;
use datadog_api_client::datadogV2::model::ObservabilityPipelineOcsfMapperProcessorType;
use datadog_api_client::datadogV2::model::ObservabilityPipelineOcsfMappingLibrary;
use datadog_api_client::datadogV2::model::ObservabilityPipelineSpec;
use datadog_api_client::datadogV2::model::ObservabilityPipelineSpecData;

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
                                    ObservabilityPipelineConfigProcessorItem::ObservabilityPipelineOcsfMapperProcessor(
                                        Box::new(
                                            ObservabilityPipelineOcsfMapperProcessor::new(
                                                true,
                                                "ocsf-mapper-processor".to_string(),
                                                "service:my-service".to_string(),
                                                vec![
                                                    ObservabilityPipelineOcsfMapperProcessorMapping::new(
                                                        "source:cloudtrail".to_string(),
                                                        ObservabilityPipelineOcsfMapperProcessorMappingMapping
                                                        ::ObservabilityPipelineOcsfMappingLibrary(
                                                            Box::new(
                                                                ObservabilityPipelineOcsfMappingLibrary
                                                                ::CLOUDTRAIL_ACCOUNT_CHANGE,
                                                            ),
                                                        ),
                                                    )
                                                ],
                                                ObservabilityPipelineOcsfMapperProcessorType::OCSF_MAPPER,
                                            ),
                                        ),
                                    )
                                ],
                            )
                        ],
                    ),
                    "OCSF Mapper Pipeline".to_string(),
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
