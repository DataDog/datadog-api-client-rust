// Validate an observability pipeline with HTTP server source valid_tokens returns
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
use datadog_api_client::datadogV2::model::ObservabilityPipelineHttpServerSource;
use datadog_api_client::datadogV2::model::ObservabilityPipelineHttpServerSourceAuthStrategy;
use datadog_api_client::datadogV2::model::ObservabilityPipelineHttpServerSourceType;
use datadog_api_client::datadogV2::model::ObservabilityPipelineHttpServerSourceValidToken;
use datadog_api_client::datadogV2::model::ObservabilityPipelineHttpServerSourceValidTokenPathToToken;
use datadog_api_client::datadogV2::model::ObservabilityPipelineHttpServerSourceValidTokenPathToTokenHeader;
use datadog_api_client::datadogV2::model::ObservabilityPipelineHttpServerSourceValidTokenPathToTokenLocation;
use datadog_api_client::datadogV2::model::ObservabilityPipelineSourceValidTokenFieldToAdd;
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
                            ObservabilityPipelineConfigSourceItem::ObservabilityPipelineHttpServerSource(
                                Box::new(
                                    ObservabilityPipelineHttpServerSource::new(
                                        ObservabilityPipelineHttpServerSourceAuthStrategy::NONE,
                                        ObservabilityPipelineDecoding::DECODE_JSON,
                                        "http-server-source".to_string(),
                                        ObservabilityPipelineHttpServerSourceType::HTTP_SERVER,
                                    ).valid_tokens(
                                        vec![
                                            ObservabilityPipelineHttpServerSourceValidToken::new(
                                                "HTTP_SERVER_TOKEN".to_string(),
                                            )
                                                .enabled(true)
                                                .field_to_add(
                                                    ObservabilityPipelineSourceValidTokenFieldToAdd::new(
                                                        "token_name".to_string(),
                                                        "primary_token".to_string(),
                                                    ),
                                                )
                                                .path_to_token(
                                                    ObservabilityPipelineHttpServerSourceValidTokenPathToToken
                                                    ::ObservabilityPipelineHttpServerSourceValidTokenPathToTokenHeader(
                                                        Box::new(
                                                            ObservabilityPipelineHttpServerSourceValidTokenPathToTokenHeader
                                                            ::new(
                                                                "X-Token".to_string(),
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ObservabilityPipelineHttpServerSourceValidToken::new(
                                                "HTTP_SERVER_TOKEN_BACKUP".to_string(),
                                            )
                                                .enabled(true)
                                                .path_to_token(
                                                    ObservabilityPipelineHttpServerSourceValidTokenPathToToken
                                                    ::ObservabilityPipelineHttpServerSourceValidTokenPathToTokenLocation(
                                                        Box::new(
                                                            ObservabilityPipelineHttpServerSourceValidTokenPathToTokenLocation
                                                            ::PATH,
                                                        ),
                                                    ),
                                                )
                                        ],
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
                                vec!["http-server-source".to_string()],
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
                    "Pipeline with HTTP server valid_tokens".to_string(),
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
