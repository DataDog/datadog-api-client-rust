// Create a ci-tests formula and functions monitor returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_monitors::MonitorsAPI;
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
        Monitor::new(
            r#"formula("query1 / query2 * 100").last("15m") >= 0.8"#.to_string(),
            MonitorType::CI_TESTS_ALERT,
        )
            .message("some message Notify: @hipchat-channel".to_string())
            .name("Example-Monitor".to_string())
            .options(
                MonitorOptions::new()
                    .thresholds(MonitorThresholds::new().critical(0.8))
                    .variables(
                        vec![
                            MonitorFormulaAndFunctionQueryDefinition::MonitorFormulaAndFunctionEventQueryDefinition(
                                Box::new(
                                    MonitorFormulaAndFunctionEventQueryDefinition::new(
                                        MonitorFormulaAndFunctionEventQueryDefinitionCompute::new(
                                            MonitorFormulaAndFunctionEventAggregation::COUNT,
                                        ),
                                        MonitorFormulaAndFunctionEventsDataSource::CI_TESTS,
                                        "query1".to_string(),
                                    )
                                        .group_by(vec![])
                                        .indexes(vec!["*".to_string()])
                                        .search(
                                            MonitorFormulaAndFunctionEventQueryDefinitionSearch::new(
                                                "@test.status:fail".to_string(),
                                            ),
                                        ),
                                ),
                            ),
                            MonitorFormulaAndFunctionQueryDefinition::MonitorFormulaAndFunctionEventQueryDefinition(
                                Box::new(
                                    MonitorFormulaAndFunctionEventQueryDefinition::new(
                                        MonitorFormulaAndFunctionEventQueryDefinitionCompute::new(
                                            MonitorFormulaAndFunctionEventAggregation::COUNT,
                                        ),
                                        MonitorFormulaAndFunctionEventsDataSource::CI_TESTS,
                                        "query2".to_string(),
                                    )
                                        .group_by(vec![])
                                        .indexes(vec!["*".to_string()])
                                        .search(
                                            MonitorFormulaAndFunctionEventQueryDefinitionSearch::new("".to_string()),
                                        ),
                                ),
                            )
                        ],
                    ),
            )
            .priority(Some(3))
            .tags(vec!["test:examplemonitor".to_string(), "env:ci".to_string()]);
    let configuration = Configuration::new();
    let api = MonitorsAPI::with_config(configuration);
    let resp = api.create_monitor(body).await;
    if let Ok(Some(value)) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
