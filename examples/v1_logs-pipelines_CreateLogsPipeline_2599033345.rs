// Create a pipeline with nested pipeline processor returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV1::api_logs_pipelines::LogsPipelinesAPI;
use datadog_api_client::datadogV1::model::LogsFilter;
use datadog_api_client::datadogV1::model::LogsPipeline;
use datadog_api_client::datadogV1::model::LogsPipelineProcessor;
use datadog_api_client::datadogV1::model::LogsPipelineProcessorType;
use datadog_api_client::datadogV1::model::LogsProcessor;

#[tokio::main]
async fn main() {
    let body = LogsPipeline::new("testPipelineWithNested".to_string())
        .description("Pipeline containing nested processor with tags and description".to_string())
        .filter(LogsFilter::new().query("source:python".to_string()))
        .processors(vec![LogsProcessor::LogsPipelineProcessor(Box::new(
            LogsPipelineProcessor::new(LogsPipelineProcessorType::PIPELINE)
                .description("This is a nested pipeline for production logs".to_string())
                .filter(LogsFilter::new().query("env:production".to_string()))
                .is_enabled(true)
                .name("nested_pipeline_with_metadata".to_string())
                .tags(vec!["env:prod".to_string(), "type:nested".to_string()]),
        ))])
        .tags(vec!["team:test".to_string()]);
    let configuration = datadog::Configuration::new();
    let api = LogsPipelinesAPI::with_config(configuration);
    let resp = api.create_logs_pipeline(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
