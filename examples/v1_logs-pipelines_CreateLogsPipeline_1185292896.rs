// Create a pipeline with Array Map Processor using category sub-processor returns
// "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV1::api_logs_pipelines::LogsPipelinesAPI;
use datadog_api_client::datadogV1::model::LogsArrayMapCategorySubProcessor;
use datadog_api_client::datadogV1::model::LogsArrayMapProcessor;
use datadog_api_client::datadogV1::model::LogsArrayMapProcessorType;
use datadog_api_client::datadogV1::model::LogsArrayMapSubProcessor;
use datadog_api_client::datadogV1::model::LogsCategoryProcessorCategory;
use datadog_api_client::datadogV1::model::LogsCategoryProcessorType;
use datadog_api_client::datadogV1::model::LogsFilter;
use datadog_api_client::datadogV1::model::LogsPipeline;
use datadog_api_client::datadogV1::model::LogsProcessor;

#[tokio::main]
async fn main() {
    let body = LogsPipeline::new("testPipelineArrayMapCategory".to_string())
        .filter(LogsFilter::new().query("source:python".to_string()))
        .processors(vec![LogsProcessor::LogsArrayMapProcessor(Box::new(
            LogsArrayMapProcessor::new(
                vec![LogsArrayMapSubProcessor::LogsArrayMapCategorySubProcessor(
                    Box::new(LogsArrayMapCategorySubProcessor::new(
                        vec![
                            LogsCategoryProcessorCategory::new()
                                .filter(
                                    LogsFilter::new()
                                        .query("@$sourceElem.status:error".to_string()),
                                )
                                .name("error".to_string()),
                            LogsCategoryProcessorCategory::new()
                                .filter(LogsFilter::new().query("*".to_string()))
                                .name("info".to_string()),
                        ],
                        "$targetElem.level".to_string(),
                        LogsCategoryProcessorType::CATEGORY_PROCESSOR,
                    )),
                )],
                "items".to_string(),
                "out".to_string(),
                LogsArrayMapProcessorType::ARRAY_MAP_PROCESSOR,
            )
            .is_enabled(true)
            .name("categorize items".to_string()),
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
