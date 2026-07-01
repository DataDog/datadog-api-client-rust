// Validate an observability pipeline with amazon_s3_generic destination SSE-KMS
// encryption returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_observability_pipelines::ObservabilityPipelinesAPI;
use datadog_api_client::datadogV2::model::ObservabilityPipelineAmazonS3DestinationServerSideEncryption;
use datadog_api_client::datadogV2::model::ObservabilityPipelineAmazonS3DestinationStorageClass;
use datadog_api_client::datadogV2::model::ObservabilityPipelineAmazonS3GenericCompression;
use datadog_api_client::datadogV2::model::ObservabilityPipelineAmazonS3GenericCompressionGzip;
use datadog_api_client::datadogV2::model::ObservabilityPipelineAmazonS3GenericCompressionGzipType;
use datadog_api_client::datadogV2::model::ObservabilityPipelineAmazonS3GenericDestination;
use datadog_api_client::datadogV2::model::ObservabilityPipelineAmazonS3GenericDestinationType;
use datadog_api_client::datadogV2::model::ObservabilityPipelineAmazonS3GenericEncoding;
use datadog_api_client::datadogV2::model::ObservabilityPipelineAmazonS3GenericEncodingJson;
use datadog_api_client::datadogV2::model::ObservabilityPipelineAmazonS3GenericEncodingJsonType;
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
                            ObservabilityPipelineConfigDestinationItem::ObservabilityPipelineAmazonS3GenericDestination(
                                Box::new(
                                    ObservabilityPipelineAmazonS3GenericDestination::new(
                                        "my-bucket".to_string(),
                                        ObservabilityPipelineAmazonS3GenericCompression
                                        ::ObservabilityPipelineAmazonS3GenericCompressionGzip(
                                            Box::new(
                                                ObservabilityPipelineAmazonS3GenericCompressionGzip::new(
                                                    ObservabilityPipelineAmazonS3GenericCompressionGzipType::GZIP,
                                                    6,
                                                ),
                                            ),
                                        ),
                                        ObservabilityPipelineAmazonS3GenericEncoding
                                        ::ObservabilityPipelineAmazonS3GenericEncodingJson(
                                            Box::new(
                                                ObservabilityPipelineAmazonS3GenericEncodingJson::new(
                                                    ObservabilityPipelineAmazonS3GenericEncodingJsonType::JSON,
                                                ),
                                            ),
                                        ),
                                        "generic-s3-destination".to_string(),
                                        vec!["my-processor-group".to_string()],
                                        "us-east-1".to_string(),
                                        ObservabilityPipelineAmazonS3DestinationStorageClass::STANDARD,
                                        ObservabilityPipelineAmazonS3GenericDestinationType::GENERIC_ARCHIVES_S3,
                                    )
                                        .server_side_encryption(
                                            ObservabilityPipelineAmazonS3DestinationServerSideEncryption::AWS_KMS,
                                        )
                                        .ssekms_key_id(
                                            "arn:aws:kms:us-east-1:123456789012:key/mrk-abc123".to_string(),
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
                    "Pipeline with S3 Generic SSE-KMS".to_string(),
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
