// Validate an observability pipeline with parse grok processor include rules
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
use datadog_api_client::datadogV2::model::ObservabilityPipelineDatadogLogsDestination;
use datadog_api_client::datadogV2::model::ObservabilityPipelineDatadogLogsDestinationType;
use datadog_api_client::datadogV2::model::ObservabilityPipelineParseGrokProcessor;
use datadog_api_client::datadogV2::model::ObservabilityPipelineParseGrokProcessorIncludeRule;
use datadog_api_client::datadogV2::model::ObservabilityPipelineParseGrokProcessorRuleItem;
use datadog_api_client::datadogV2::model::ObservabilityPipelineParseGrokProcessorRuleMatchRule;
use datadog_api_client::datadogV2::model::ObservabilityPipelineParseGrokProcessorType;
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
                                    ObservabilityPipelineConfigProcessorItem::ObservabilityPipelineParseGrokProcessor(
                                        Box::new(
                                            ObservabilityPipelineParseGrokProcessor::new(
                                                true,
                                                "parse-grok-processor".to_string(),
                                                "*".to_string(),
                                                vec![
                                                    ObservabilityPipelineParseGrokProcessorRuleItem
                                                    ::ObservabilityPipelineParseGrokProcessorIncludeRule(
                                                        Box::new(
                                                            ObservabilityPipelineParseGrokProcessorIncludeRule::new(
                                                                "service:foo".to_string(),
                                                                vec![
                                                                    ObservabilityPipelineParseGrokProcessorRuleMatchRule
                                                                    ::new(
                                                                        "MyParsingRule".to_string(),
                                                                        "%{word:user}".to_string(),
                                                                    )
                                                                ],
                                                            ),
                                                        ),
                                                    )
                                                ],
                                                ObservabilityPipelineParseGrokProcessorType::PARSE_GROK,
                                            ).field("content".to_string()),
                                        ),
                                    )
                                ],
                            )
                        ],
                    ),
                    "Pipeline with Parse Grok Include Rules".to_string(),
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
