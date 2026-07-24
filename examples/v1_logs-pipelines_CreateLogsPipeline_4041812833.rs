// Create a pipeline with Array Processor Key Value Operation with target and
// override_on_conflict returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV1::api_logs_pipelines::LogsPipelinesAPI;
use datadog_api_client::datadogV1::model::LogsArrayProcessor;
use datadog_api_client::datadogV1::model::LogsArrayProcessorOperation;
use datadog_api_client::datadogV1::model::LogsArrayProcessorOperationExtractKeyValue;
use datadog_api_client::datadogV1::model::LogsArrayProcessorOperationExtractKeyValueType;
use datadog_api_client::datadogV1::model::LogsArrayProcessorType;
use datadog_api_client::datadogV1::model::LogsFilter;
use datadog_api_client::datadogV1::model::LogsPipeline;
use datadog_api_client::datadogV1::model::LogsProcessor;

#[tokio::main]
async fn main() {
    let body = LogsPipeline::new("testPipelineArrayKeyValueTarget".to_string())
        .filter(LogsFilter::new().query("source:python".to_string()))
        .processors(vec![LogsProcessor::LogsArrayProcessor(Box::new(
            LogsArrayProcessor::new(
                LogsArrayProcessorOperation::LogsArrayProcessorOperationExtractKeyValue(Box::new(
                    LogsArrayProcessorOperationExtractKeyValue::new(
                        "name".to_string(),
                        "tags".to_string(),
                        LogsArrayProcessorOperationExtractKeyValueType::KEY_VALUE,
                        "value".to_string(),
                    )
                    .override_on_conflict(true)
                    .target("extracted".to_string()),
                )),
                LogsArrayProcessorType::ARRAY_PROCESSOR,
            )
            .is_enabled(true)
            .name("extract_kv_to_target".to_string()),
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
