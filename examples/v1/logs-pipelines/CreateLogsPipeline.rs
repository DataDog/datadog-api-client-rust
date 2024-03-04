// Create a pipeline returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_logs_pipelines::LogsPipelinesAPI;
use datadog_api_client::datadogV1::model::*;
use std::ops::Add;
use std::time::{
    Duration,
    SystemTime,
    UNIX_EPOCH,
};

#[tokio::main]
async fn main() {
    let body =
        LogsPipeline::new("".to_string())
            .filter(LogsFilter::new().query("source:python".to_string()))
            .processors(
                vec![
                    LogsProcessor::LogsGrokParser(
                        Box::new(
                            LogsGrokParser::new(
                                LogsGrokParserRules::new(
                                    r#"rule_name_1 foo
rule_name_2 bar
"#.to_string(),
                                ).support_rules(r#"rule_name_1 foo
rule_name_2 bar
"#.to_string()),
                                "message".to_string(),
                                LogsGrokParserType::GROK_PARSER,
                            )
                                .is_enabled(false)
                                .samples(vec![]),
                        ),
                    )
                ],
            );
    let configuration = Configuration::new();
    let api = LogsPipelinesAPI::with_config(configuration);
    let resp = api.create_logs_pipeline(body).await;
    if let Ok(Some(value)) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
