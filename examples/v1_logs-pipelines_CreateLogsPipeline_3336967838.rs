// Create a pipeline with Decoder Processor returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV1::api_logs_pipelines::LogsPipelinesAPI;
use datadog_api_client::datadogV1::model::LogsDecoderProcessor;
use datadog_api_client::datadogV1::model::LogsDecoderProcessorBinaryToTextEncoding;
use datadog_api_client::datadogV1::model::LogsDecoderProcessorInputRepresentation;
use datadog_api_client::datadogV1::model::LogsDecoderProcessorType;
use datadog_api_client::datadogV1::model::LogsFilter;
use datadog_api_client::datadogV1::model::LogsPipeline;
use datadog_api_client::datadogV1::model::LogsProcessor;

#[tokio::main]
async fn main() {
    let body = LogsPipeline::new("testDecoderProcessor".to_string())
        .filter(LogsFilter::new().query("source:python".to_string()))
        .processors(vec![LogsProcessor::LogsDecoderProcessor(Box::new(
            LogsDecoderProcessor::new(
                LogsDecoderProcessorBinaryToTextEncoding::BASE16,
                LogsDecoderProcessorInputRepresentation::UTF_8,
                "encoded.field".to_string(),
                "decoded.field".to_string(),
                LogsDecoderProcessorType::DECODER_PROCESSOR,
            )
            .is_enabled(true)
            .name("test_decoder".to_string()),
        ))])
        .tags(vec![]);
    let configuration = datadog::Configuration::new();
    let api = LogsPipelinesAPI::with_config(configuration);
    let resp = api.create_logs_pipeline(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
