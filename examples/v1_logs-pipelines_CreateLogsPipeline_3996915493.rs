// Create a pipeline with Array Map Processor returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV1::api_logs_pipelines::LogsPipelinesAPI;
use datadog_api_client::datadogV1::model::LogsArrayMapAttributeRemapper;
use datadog_api_client::datadogV1::model::LogsArrayMapProcessor;
use datadog_api_client::datadogV1::model::LogsArrayMapProcessorType;
use datadog_api_client::datadogV1::model::LogsArrayMapStringBuilderSubProcessor;
use datadog_api_client::datadogV1::model::LogsArrayMapSubProcessor;
use datadog_api_client::datadogV1::model::LogsAttributeRemapperType;
use datadog_api_client::datadogV1::model::LogsFilter;
use datadog_api_client::datadogV1::model::LogsPipeline;
use datadog_api_client::datadogV1::model::LogsProcessor;
use datadog_api_client::datadogV1::model::LogsStringBuilderProcessorType;

#[tokio::main]
async fn main() {
    let body = LogsPipeline::new("testPipelineArrayMap".to_string())
        .filter(LogsFilter::new().query("source:python".to_string()))
        .processors(vec![LogsProcessor::LogsArrayMapProcessor(Box::new(
            LogsArrayMapProcessor::new(
                vec![
                    LogsArrayMapSubProcessor::LogsArrayMapAttributeRemapper(Box::new(
                        LogsArrayMapAttributeRemapper::new(
                            vec!["$sourceElem.id".to_string()],
                            "$targetElem.uid".to_string(),
                            LogsAttributeRemapperType::ATTRIBUTE_REMAPPER,
                        )
                        .preserve_source(true),
                    )),
                    LogsArrayMapSubProcessor::LogsArrayMapStringBuilderSubProcessor(Box::new(
                        LogsArrayMapStringBuilderSubProcessor::new(
                            "$targetElem.label".to_string(),
                            "item-%{$sourceElem.id}".to_string(),
                            LogsStringBuilderProcessorType::STRING_BUILDER_PROCESSOR,
                        ),
                    )),
                ],
                "items".to_string(),
                "out".to_string(),
                LogsArrayMapProcessorType::ARRAY_MAP_PROCESSOR,
            )
            .is_enabled(true)
            .name("map items".to_string())
            .preserve_source(true),
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
