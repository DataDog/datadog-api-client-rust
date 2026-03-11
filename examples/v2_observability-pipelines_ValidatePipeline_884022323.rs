// Validate a metrics pipeline with opentelemetry source returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_observability_pipelines::ObservabilityPipelinesAPI;
use datadog_api_client::datadogV2::model::ObservabilityPipelineConfig;
use datadog_api_client::datadogV2::model::ObservabilityPipelineConfigDestinationItem;
use datadog_api_client::datadogV2::model::ObservabilityPipelineConfigPipelineType;
use datadog_api_client::datadogV2::model::ObservabilityPipelineConfigProcessorGroup;
use datadog_api_client::datadogV2::model::ObservabilityPipelineConfigProcessorItem;
use datadog_api_client::datadogV2::model::ObservabilityPipelineConfigSourceItem;
use datadog_api_client::datadogV2::model::ObservabilityPipelineDataAttributes;
use datadog_api_client::datadogV2::model::ObservabilityPipelineDatadogMetricsDestination;
use datadog_api_client::datadogV2::model::ObservabilityPipelineDatadogMetricsDestinationType;
use datadog_api_client::datadogV2::model::ObservabilityPipelineFilterProcessor;
use datadog_api_client::datadogV2::model::ObservabilityPipelineFilterProcessorType;
use datadog_api_client::datadogV2::model::ObservabilityPipelineOpentelemetrySource;
use datadog_api_client::datadogV2::model::ObservabilityPipelineOpentelemetrySourceType;
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
                            ObservabilityPipelineConfigSourceItem::ObservabilityPipelineOpentelemetrySource(
                                Box::new(
                                    ObservabilityPipelineOpentelemetrySource::new(
                                        "opentelemetry-source".to_string(),
                                        ObservabilityPipelineOpentelemetrySourceType::OPENTELEMETRY,
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
                                    vec!["opentelemetry-source".to_string()],
                                    vec![
                                        ObservabilityPipelineConfigProcessorItem::ObservabilityPipelineFilterProcessor(
                                            Box::new(
                                                ObservabilityPipelineFilterProcessor::new(
                                                    true,
                                                    "filter-processor".to_string(),
                                                    "env:production".to_string(),
                                                    ObservabilityPipelineFilterProcessorType::FILTER,
                                                ),
                                            ),
                                        )
                                    ],
                                )
                            ],
                        ),
                    "Metrics OTel Pipeline".to_string(),
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
