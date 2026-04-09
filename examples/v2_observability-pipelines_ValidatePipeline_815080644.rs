// Validate an observability pipeline with enrichment table secret field lookup
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
use datadog_api_client::datadogV2::model::ObservabilityPipelineEnrichmentTableFieldSecretLookup;
use datadog_api_client::datadogV2::model::ObservabilityPipelineEnrichmentTableFile;
use datadog_api_client::datadogV2::model::ObservabilityPipelineEnrichmentTableFileEncoding;
use datadog_api_client::datadogV2::model::ObservabilityPipelineEnrichmentTableFileEncodingType;
use datadog_api_client::datadogV2::model::ObservabilityPipelineEnrichmentTableFileKeyItemField;
use datadog_api_client::datadogV2::model::ObservabilityPipelineEnrichmentTableFileKeyItems;
use datadog_api_client::datadogV2::model::ObservabilityPipelineEnrichmentTableFileKeyItemsComparison;
use datadog_api_client::datadogV2::model::ObservabilityPipelineEnrichmentTableFileSchemaItems;
use datadog_api_client::datadogV2::model::ObservabilityPipelineEnrichmentTableFileSchemaItemsType;
use datadog_api_client::datadogV2::model::ObservabilityPipelineEnrichmentTableProcessor;
use datadog_api_client::datadogV2::model::ObservabilityPipelineEnrichmentTableProcessorType;
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
                                    ObservabilityPipelineConfigProcessorItem
                                    ::ObservabilityPipelineEnrichmentTableProcessor(
                                        Box::new(
                                            ObservabilityPipelineEnrichmentTableProcessor::new(
                                                true,
                                                "enrichment-processor".to_string(),
                                                "*".to_string(),
                                                "enriched".to_string(),
                                                ObservabilityPipelineEnrichmentTableProcessorType::ENRICHMENT_TABLE,
                                            ).file(
                                                ObservabilityPipelineEnrichmentTableFile::new(
                                                    ObservabilityPipelineEnrichmentTableFileEncoding::new(
                                                        ",".to_string(),
                                                        true,
                                                        ObservabilityPipelineEnrichmentTableFileEncodingType::CSV,
                                                    ),
                                                    vec![
                                                        ObservabilityPipelineEnrichmentTableFileKeyItems::new(
                                                            "user_id".to_string(),
                                                            ObservabilityPipelineEnrichmentTableFileKeyItemsComparison
                                                            ::EQUALS,
                                                            ObservabilityPipelineEnrichmentTableFileKeyItemField
                                                            ::ObservabilityPipelineEnrichmentTableFieldSecretLookup(
                                                                Box::new(
                                                                    ObservabilityPipelineEnrichmentTableFieldSecretLookup
                                                                    ::new(
                                                                        "LOOKUP_KEY_SECRET".to_string(),
                                                                    ),
                                                                ),
                                                            ),
                                                        )
                                                    ],
                                                    "/etc/enrichment/lookup.csv".to_string(),
                                                    vec![
                                                        ObservabilityPipelineEnrichmentTableFileSchemaItems::new(
                                                            "user_id".to_string(),
                                                            ObservabilityPipelineEnrichmentTableFileSchemaItemsType
                                                            ::STRING,
                                                        )
                                                    ],
                                                ),
                                            ),
                                        ),
                                    )
                                ],
                            )
                        ],
                    ),
                    "Pipeline with Enrichment Table Secret Field Lookup".to_string(),
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
