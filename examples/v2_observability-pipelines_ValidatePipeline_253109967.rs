// Validate an observability pipeline with ClickHouse destination with all fields
// set returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_observability_pipelines::ObservabilityPipelinesAPI;
use datadog_api_client::datadogV2::model::ObservabilityPipelineBufferOptions;
use datadog_api_client::datadogV2::model::ObservabilityPipelineBufferOptionsMemoryType;
use datadog_api_client::datadogV2::model::ObservabilityPipelineBufferOptionsWhenFull;
use datadog_api_client::datadogV2::model::ObservabilityPipelineClickhouseDestination;
use datadog_api_client::datadogV2::model::ObservabilityPipelineClickhouseDestinationAuth;
use datadog_api_client::datadogV2::model::ObservabilityPipelineClickhouseDestinationAuthStrategy;
use datadog_api_client::datadogV2::model::ObservabilityPipelineClickhouseDestinationBatch;
use datadog_api_client::datadogV2::model::ObservabilityPipelineClickhouseDestinationBatchEncoding;
use datadog_api_client::datadogV2::model::ObservabilityPipelineClickhouseDestinationBatchEncodingCodec;
use datadog_api_client::datadogV2::model::ObservabilityPipelineClickhouseDestinationCompression;
use datadog_api_client::datadogV2::model::ObservabilityPipelineClickhouseDestinationCompressionAlgorithm;
use datadog_api_client::datadogV2::model::ObservabilityPipelineClickhouseDestinationCompressionObject;
use datadog_api_client::datadogV2::model::ObservabilityPipelineClickhouseDestinationFormat;
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
use datadog_api_client::datadogV2::model::ObservabilityPipelineMemoryBufferSizeOptions;
use datadog_api_client::datadogV2::model::ObservabilityPipelineSpec;
use datadog_api_client::datadogV2::model::ObservabilityPipelineSpecData;
use datadog_api_client::datadogV2::model::ObservabilityPipelineTls;

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
                                        .batch_encoding(
                                            ObservabilityPipelineClickhouseDestinationBatchEncoding::new(
                                                ObservabilityPipelineClickhouseDestinationBatchEncodingCodec
                                                ::ARROW_STREAM,
                                            ).allow_nullable_fields(true),
                                        )
                                        .buffer(
                                            ObservabilityPipelineBufferOptions
                                            ::ObservabilityPipelineMemoryBufferSizeOptions(
                                                Box::new(
                                                    ObservabilityPipelineMemoryBufferSizeOptions::new(500)
                                                        .type_(ObservabilityPipelineBufferOptionsMemoryType::MEMORY)
                                                        .when_full(ObservabilityPipelineBufferOptionsWhenFull::BLOCK),
                                                ),
                                            ),
                                        )
                                        .compression(
                                            ObservabilityPipelineClickhouseDestinationCompression
                                            ::ObservabilityPipelineClickhouseDestinationCompressionObject(
                                                Box::new(
                                                    ObservabilityPipelineClickhouseDestinationCompressionObject::new(
                                                        ObservabilityPipelineClickhouseDestinationCompressionAlgorithm
                                                        ::GZIP,
                                                    ).level(6),
                                                ),
                                            ),
                                        )
                                        .database("my_database".to_string())
                                        .date_time_best_effort(true)
                                        .endpoint_url_key("CLICKHOUSE_ENDPOINT_URL".to_string())
                                        .format(ObservabilityPipelineClickhouseDestinationFormat::ARROW_STREAM)
                                        .skip_unknown_fields(Some(true))
                                        .tls(
                                            ObservabilityPipelineTls::new("/path/to/cert.crt".to_string())
                                                .ca_file("/path/to/ca.crt".to_string())
                                                .key_file("/path/to/key.key".to_string())
                                                .key_pass_key("TLS_KEY_PASSPHRASE".to_string()),
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
                    "Pipeline with ClickHouse Destination All Fields".to_string(),
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
