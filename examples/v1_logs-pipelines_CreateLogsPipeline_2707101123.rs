// Create a pipeline with Span Id Remapper returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV1::api_logs_pipelines::LogsPipelinesAPI;
use datadog_api_client::datadogV1::model::LogsFilter;
use datadog_api_client::datadogV1::model::LogsPipeline;
use datadog_api_client::datadogV1::model::LogsProcessor;
use datadog_api_client::datadogV1::model::LogsSpanRemapper;
use datadog_api_client::datadogV1::model::LogsSpanRemapperType;

#[tokio::main]
async fn main() {
    let body = LogsPipeline::new("testPipeline".to_string())
        .filter(LogsFilter::new().query("source:python".to_string()))
        .processors(vec![LogsProcessor::LogsSpanRemapper(Box::new(
            LogsSpanRemapper::new(LogsSpanRemapperType::SPAN_ID_REMAPPER)
                .is_enabled(true)
                .name("test_filter".to_string())
                .sources(vec!["dd.span_id".to_string()]),
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
