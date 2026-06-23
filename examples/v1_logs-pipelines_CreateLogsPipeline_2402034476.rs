// Create a pipeline with Array Map Processor using arithmetic sub-processor
// returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV1::api_logs_pipelines::LogsPipelinesAPI;
use datadog_api_client::datadogV1::model::LogsArithmeticProcessorType;
use datadog_api_client::datadogV1::model::LogsArrayMapArithmeticSubProcessor;
use datadog_api_client::datadogV1::model::LogsArrayMapProcessor;
use datadog_api_client::datadogV1::model::LogsArrayMapProcessorType;
use datadog_api_client::datadogV1::model::LogsArrayMapSubProcessor;
use datadog_api_client::datadogV1::model::LogsFilter;
use datadog_api_client::datadogV1::model::LogsPipeline;
use datadog_api_client::datadogV1::model::LogsProcessor;

#[tokio::main]
async fn main() {
    let body = LogsPipeline::new("testPipelineArrayMapArithmetic".to_string())
        .filter(LogsFilter::new().query("source:python".to_string()))
        .processors(vec![LogsProcessor::LogsArrayMapProcessor(Box::new(
            LogsArrayMapProcessor::new(
                vec![
                    LogsArrayMapSubProcessor::LogsArrayMapArithmeticSubProcessor(Box::new(
                        LogsArrayMapArithmeticSubProcessor::new(
                            "$sourceElem.count * 2".to_string(),
                            "$targetElem.doubled".to_string(),
                            LogsArithmeticProcessorType::ARITHMETIC_PROCESSOR,
                        ),
                    )),
                ],
                "items".to_string(),
                "out".to_string(),
                LogsArrayMapProcessorType::ARRAY_MAP_PROCESSOR,
            )
            .is_enabled(true)
            .name("double counts".to_string()),
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
