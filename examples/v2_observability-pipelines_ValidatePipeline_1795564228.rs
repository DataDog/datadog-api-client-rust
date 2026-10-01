// Validate a metrics pipeline with enrichment table processor file lookup returns
// "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_observability_pipelines::ObservabilityPipelinesAPI;
use datadog_api_client::datadogV2::model::ObservabilityPipelineConfig;
use datadog_api_client::datadogV2::model::ObservabilityPipelineConfigDestinationItem;
use datadog_api_client::datadogV2::model::ObservabilityPipelineConfigPipelineType;
use datadog_api_client::datadogV2::model::ObservabilityPipelineConfigProcessorGroup;
use datadog_api_client::datadogV2::model::ObservabilityPipelineConfigProcessorItem;
use datadog_api_client::datadogV2::model::ObservabilityPipelineConfigSourceItem;
use datadog_api_client::datadogV2::model::ObservabilityPipelineDataAttributes;
use datadog_api_client::datadogV2::model::ObservabilityPipelineDatadogAgentSource;
use datadog_api_client::datadogV2::model::ObservabilityPipelineDatadogAgentSourceType;
use datadog_api_client::datadogV2::model::ObservabilityPipelineDatadogMetricsDestination;
use datadog_api_client::datadogV2::model::ObservabilityPipelineDatadogMetricsDestinationType;
use datadog_api_client::datadogV2::model::ObservabilityPipelineEnrichmentTableFileEncoding;
use datadog_api_client::datadogV2::model::ObservabilityPipelineEnrichmentTableFileEncodingType;
use datadog_api_client::datadogV2::model::ObservabilityPipelineEnrichmentTableProcessorType;
use datadog_api_client::datadogV2::model::ObservabilityPipelineMetricEnrichmentTableFile;
use datadog_api_client::datadogV2::model::ObservabilityPipelineMetricEnrichmentTableFileKey;
use datadog_api_client::datadogV2::model::ObservabilityPipelineMetricEnrichmentTableFileProcessor;
use datadog_api_client::datadogV2::model::ObservabilityPipelineMetricEnrichmentTableLookupSource;
use datadog_api_client::datadogV2::model::ObservabilityPipelineMetricEnrichmentTableProcessor;
use datadog_api_client::datadogV2::model::ObservabilityPipelineMetricEnrichmentTableTagLookup;
use datadog_api_client::datadogV2::model::ObservabilityPipelineMetricEnrichmentTableTagLookupType;
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
                            ObservabilityPipelineConfigDestinationItem::ObservabilityPipelineDatadogMetricsDestination(
                                Box::new(
                                    ObservabilityPipelineDatadogMetricsDestination::new(
                                        "datadog-metrics-destination".to_string(),
                                        vec!["my-processor-group".to_string()],
                                        ObservabilityPipelineDatadogMetricsDestinationType::DATADOG_METRICS,
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
                    )
                        .pipeline_type(ObservabilityPipelineConfigPipelineType::METRICS)
                        .processor_groups(
                            vec![
                                ObservabilityPipelineConfigProcessorGroup::new(
                                    true,
                                    "my-processor-group".to_string(),
                                    "*".to_string(),
                                    vec!["datadog-agent-source".to_string()],
                                    vec![
                                        ObservabilityPipelineConfigProcessorItem
                                        ::ObservabilityPipelineMetricEnrichmentTableProcessor(
                                            Box::new(
                                                ObservabilityPipelineMetricEnrichmentTableProcessor
                                                ::ObservabilityPipelineMetricEnrichmentTableFileProcessor(
                                                    Box::new(
                                                        ObservabilityPipelineMetricEnrichmentTableFileProcessor::new(
                                                            true,
                                                            ObservabilityPipelineMetricEnrichmentTableFile::new(
                                                                ObservabilityPipelineEnrichmentTableFileEncoding::new(
                                                                    ",".to_string(),
                                                                    true,
                                                                    ObservabilityPipelineEnrichmentTableFileEncodingType
                                                                    ::CSV,
                                                                ),
                                                                ObservabilityPipelineMetricEnrichmentTableFileKey::new(
                                                                    "service".to_string(),
                                                                    ObservabilityPipelineMetricEnrichmentTableLookupSource
                                                                    ::ObservabilityPipelineMetricEnrichmentTableTagLookup(
                                                                        Box::new(
                                                                            ObservabilityPipelineMetricEnrichmentTableTagLookup
                                                                            ::new(
                                                                                "service".to_string(),
                                                                                ObservabilityPipelineMetricEnrichmentTableTagLookupType
                                                                                ::TAG,
                                                                            ),
                                                                        ),
                                                                    ),
                                                                ),
                                                                "/etc/enrichment/lookup.csv".to_string(),
                                                            ),
                                                            "enrichment-table-processor".to_string(),
                                                            "*".to_string(),
                                                            ObservabilityPipelineEnrichmentTableProcessorType
                                                            ::ENRICHMENT_TABLE,
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        )
                                    ],
                                )
                            ],
                        ),
                    "Metrics Pipeline with Enrichment Table File Lookup".to_string(),
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
