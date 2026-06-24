// Validate an observability pipeline with websocket source bearer auth returns
// "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_observability_pipelines::ObservabilityPipelinesAPI;
use datadog_api_client::datadogV2::model::ObservabilityPipelineConfig;
use datadog_api_client::datadogV2::model::ObservabilityPipelineConfigDestinationItem;
use datadog_api_client::datadogV2::model::ObservabilityPipelineConfigProcessorGroup;
use datadog_api_client::datadogV2::model::ObservabilityPipelineConfigProcessorItem;
use datadog_api_client::datadogV2::model::ObservabilityPipelineConfigSourceItem;
use datadog_api_client::datadogV2::model::ObservabilityPipelineDataAttributes;
use datadog_api_client::datadogV2::model::ObservabilityPipelineDatadogLogsDestination;
use datadog_api_client::datadogV2::model::ObservabilityPipelineDatadogLogsDestinationType;
use datadog_api_client::datadogV2::model::ObservabilityPipelineDecoding;
use datadog_api_client::datadogV2::model::ObservabilityPipelineFilterProcessor;
use datadog_api_client::datadogV2::model::ObservabilityPipelineFilterProcessorType;
use datadog_api_client::datadogV2::model::ObservabilityPipelineSpec;
use datadog_api_client::datadogV2::model::ObservabilityPipelineSpecData;
use datadog_api_client::datadogV2::model::ObservabilityPipelineWebsocketSource;
use datadog_api_client::datadogV2::model::ObservabilityPipelineWebsocketSourceAuthStrategy;
use datadog_api_client::datadogV2::model::ObservabilityPipelineWebsocketSourceTls;
use datadog_api_client::datadogV2::model::ObservabilityPipelineWebsocketSourceTlsEnabled;
use datadog_api_client::datadogV2::model::ObservabilityPipelineWebsocketSourceTlsEnabledMode;
use datadog_api_client::datadogV2::model::ObservabilityPipelineWebsocketSourceType;

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
                            ObservabilityPipelineConfigSourceItem::ObservabilityPipelineWebsocketSource(
                                Box::new(
                                    ObservabilityPipelineWebsocketSource::new(
                                        ObservabilityPipelineWebsocketSourceAuthStrategy::BEARER,
                                        ObservabilityPipelineDecoding::DECODE_JSON,
                                        "websocket-source".to_string(),
                                        ObservabilityPipelineWebsocketSourceType::WEBSOCKET,
                                    )
                                        .tls(
                                            ObservabilityPipelineWebsocketSourceTls
                                            ::ObservabilityPipelineWebsocketSourceTlsEnabled(
                                                Box::new(
                                                    ObservabilityPipelineWebsocketSourceTlsEnabled::new(
                                                        ObservabilityPipelineWebsocketSourceTlsEnabledMode::ENABLED,
                                                    ),
                                                ),
                                            ),
                                        )
                                        .token_key("WS_BEARER_TOKEN".to_string())
                                        .uri_key("WS_URI".to_string()),
                                ),
                            )
                        ],
                    ).processor_groups(
                        vec![
                            ObservabilityPipelineConfigProcessorGroup::new(
                                true,
                                "my-processor-group".to_string(),
                                "service:my-service".to_string(),
                                vec!["websocket-source".to_string()],
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
                    "Pipeline with WebSocket Source".to_string(),
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
