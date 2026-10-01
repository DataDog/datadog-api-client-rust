// Validate a metrics pipeline with enrichment table processor reference table
// returns "OK" response
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
use datadog_api_client::datadogV2::model::ObservabilityPipelineEnrichmentTableProcessorType;
use datadog_api_client::datadogV2::model::ObservabilityPipelineMetricEnrichmentTableLookupSource;
use datadog_api_client::datadogV2::model::ObservabilityPipelineMetricEnrichmentTableMetricNameLookup;
use datadog_api_client::datadogV2::model::ObservabilityPipelineMetricEnrichmentTableMetricNameLookupType;
use datadog_api_client::datadogV2::model::ObservabilityPipelineMetricEnrichmentTableProcessor;
use datadog_api_client::datadogV2::model::ObservabilityPipelineMetricEnrichmentTableReferenceKey;
use datadog_api_client::datadogV2::model::ObservabilityPipelineMetricEnrichmentTableReferenceTable;
use datadog_api_client::datadogV2::model::ObservabilityPipelineMetricEnrichmentTableReferenceTableProcessor;
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
                                                ::ObservabilityPipelineMetricEnrichmentTableReferenceTableProcessor(
                                                    Box::new(
                                                        ObservabilityPipelineMetricEnrichmentTableReferenceTableProcessor
                                                        ::new(
                                                            true,
                                                            "enrichment-table-processor".to_string(),
                                                            "*".to_string(),
                                                            ObservabilityPipelineMetricEnrichmentTableReferenceTable
                                                            ::new(
                                                                ObservabilityPipelineMetricEnrichmentTableReferenceKey
                                                                ::new(
                                                                    ObservabilityPipelineMetricEnrichmentTableLookupSource
                                                                    ::ObservabilityPipelineMetricEnrichmentTableMetricNameLookup(
                                                                        Box::new(
                                                                            ObservabilityPipelineMetricEnrichmentTableMetricNameLookup
                                                                            ::new(
                                                                                ObservabilityPipelineMetricEnrichmentTableMetricNameLookupType
                                                                                ::METRIC_NAME,
                                                                            ),
                                                                        ),
                                                                    ),
                                                                ),
                                                                "metric-enrichment".to_string(),
                                                            ).columns(
                                                                vec!["environment".to_string(), "team".to_string()],
                                                            ),
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
                    "Metrics Pipeline with Enrichment Table Reference Table".to_string(),
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
