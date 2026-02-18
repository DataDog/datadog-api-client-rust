// Validate an observability pipeline with OCSF mapper custom mapping returns "OK"
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
use datadog_api_client::datadogV2::model::ObservabilityPipelineOcsfMapperProcessor;
use datadog_api_client::datadogV2::model::ObservabilityPipelineOcsfMapperProcessorMapping;
use datadog_api_client::datadogV2::model::ObservabilityPipelineOcsfMapperProcessorMappingMapping;
use datadog_api_client::datadogV2::model::ObservabilityPipelineOcsfMapperProcessorType;
use datadog_api_client::datadogV2::model::ObservabilityPipelineOcsfMappingCustom;
use datadog_api_client::datadogV2::model::ObservabilityPipelineOcsfMappingCustomFieldMapping;
use datadog_api_client::datadogV2::model::ObservabilityPipelineOcsfMappingCustomLookup;
use datadog_api_client::datadogV2::model::ObservabilityPipelineOcsfMappingCustomLookupTableEntry;
use datadog_api_client::datadogV2::model::ObservabilityPipelineOcsfMappingCustomMetadata;
use datadog_api_client::datadogV2::model::ObservabilityPipelineSpec;
use datadog_api_client::datadogV2::model::ObservabilityPipelineSpecData;
use serde_json::Value;

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
                                                        "source:custom".to_string(),
                                                        ObservabilityPipelineOcsfMapperProcessorMappingMapping
                                                        ::ObservabilityPipelineOcsfMappingCustom(
                                                            Box::new(
                                                                ObservabilityPipelineOcsfMappingCustom::new(
                                                                    vec![
                                                                        ObservabilityPipelineOcsfMappingCustomFieldMapping
                                                                        ::new(
                                                                            "time".to_string(),
                                                                        )
                                                                            .default(Value::from(""))
                                                                            .source(Value::from("timestamp")),
                                                                        ObservabilityPipelineOcsfMappingCustomFieldMapping
                                                                        ::new(
                                                                            "severity".to_string(),
                                                                        )
                                                                            .default(Value::from(""))
                                                                            .source(Value::from("level")),
                                                                        ObservabilityPipelineOcsfMappingCustomFieldMapping
                                                                        ::new(
                                                                            "device.type".to_string(),
                                                                        )
                                                                            .default(Value::from(""))
                                                                            .lookup(
                                                                                ObservabilityPipelineOcsfMappingCustomLookup
                                                                                ::new().table(
                                                                                    vec![
                                                                                        ObservabilityPipelineOcsfMappingCustomLookupTableEntry
                                                                                        ::new()
                                                                                            .contains(
                                                                                                "Desktop".to_string(),
                                                                                            )
                                                                                            .value(
                                                                                                Value::from("desktop"),
                                                                                            )
                                                                                    ],
                                                                                ),
                                                                            )
                                                                            .source(Value::from("host.type"))
                                                                    ],
                                                                    ObservabilityPipelineOcsfMappingCustomMetadata
                                                                    ::new(
                                                                        "Device Inventory Info".to_string(),
                                                                        "1.3.0".to_string(),
                                                                    ).profiles(vec!["container".to_string()]),
                                                                    1,
                                                                ),
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
                    "OCSF Custom Mapper Pipeline".to_string(),
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
