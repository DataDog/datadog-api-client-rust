// Validate an observability pipeline with ClickHouse destination returns "OK"
// response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_observability_pipelines::ObservabilityPipelinesAPI;
use datadog_api_client::datadogV2::model::ObservabilityPipelineClickhouseDestination;
use datadog_api_client::datadogV2::model::ObservabilityPipelineClickhouseDestinationAuth;
use datadog_api_client::datadogV2::model::ObservabilityPipelineClickhouseDestinationAuthStrategy;
use datadog_api_client::datadogV2::model::ObservabilityPipelineClickhouseDestinationBatch;
use datadog_api_client::datadogV2::model::ObservabilityPipelineClickhouseDestinationCompression;
use datadog_api_client::datadogV2::model::ObservabilityPipelineClickhouseDestinationCompressionAlgorithm;
use datadog_api_client::datadogV2::model::ObservabilityPipelineClickhouseDestinationType;
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

#[tokio::main]
async fn main() {
    let body =
        ObservabilityPipelineSpec::new(
            ObservabilityPipelineSpecData::new(
                ObservabilityPipelineDataAttributes::new(
                    ObservabilityPipelineConfig::new(
                        vec![
                            ObservabilityPipelineConfigDestinationItem::ObservabilityPipelineClickhouseDestination(
                                Box::new(
                                    ObservabilityPipelineClickhouseDestination::new(
                                        "clickhouse-destination".to_string(),
                                        vec!["my-processor-group".to_string()],
                                        "application_logs".to_string(),
                                        ObservabilityPipelineClickhouseDestinationType::CLICKHOUSE,
                                    )
                                        .auth(
                                            ObservabilityPipelineClickhouseDestinationAuth::new(
                                                ObservabilityPipelineClickhouseDestinationAuthStrategy::BASIC,
                                            )
                                                .password_key("CLICKHOUSE_PASSWORD".to_string())
                                                .username_key("CLICKHOUSE_USERNAME".to_string()),
                                        )
                                        .batch(
                                            ObservabilityPipelineClickhouseDestinationBatch::new()
                                                .max_events(1000)
                                                .timeout_secs(1),
                                        )
                                        .compression(
                                            ObservabilityPipelineClickhouseDestinationCompression
                                            ::ObservabilityPipelineClickhouseDestinationCompressionAlgorithm(
                                                Box::new(
                                                    ObservabilityPipelineClickhouseDestinationCompressionAlgorithm
                                                    ::GZIP,
                                                ),
                                            ),
                                        )
                                        .database("my_database".to_string()),
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
                    "Pipeline with ClickHouse Destination".to_string(),
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
