// Create a monitor with a metrics formula variable returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV1::api_monitors::MonitorsAPI;
use datadog_api_client::datadogV1::model::Monitor;
use datadog_api_client::datadogV1::model::MonitorFormulaAndFunctionMetricQueryDataSource;
use datadog_api_client::datadogV1::model::MonitorFormulaAndFunctionMetricQueryDefinition;
use datadog_api_client::datadogV1::model::MonitorFormulaAndFunctionQueryDefinition;
use datadog_api_client::datadogV1::model::MonitorOptions;
use datadog_api_client::datadogV1::model::MonitorThresholds;
use datadog_api_client::datadogV1::model::MonitorType;

#[tokio::main]
async fn main() {
    let body =
        Monitor::new(
            r#"formula("query1 / query2").rollup("avg").last("5m") > threshold"#.to_string(),
            MonitorType::QUERY_ALERT,
        )
            .message("some message Notify: @hipchat-channel".to_string())
            .name("Example-Monitor".to_string())
            .options(
                MonitorOptions::new()
                    .thresholds(
                        MonitorThresholds
                        ::new().critical_query(
                            r#"formula("query1 / query2").rollup("p99").last("1w").multiplier(1.5)"#.to_string(),
                        ),
                    )
                    .variables(
                        vec![
                            MonitorFormulaAndFunctionQueryDefinition::MonitorFormulaAndFunctionMetricQueryDefinition(
                                Box::new(
                                    MonitorFormulaAndFunctionMetricQueryDefinition::new(
                                        MonitorFormulaAndFunctionMetricQueryDataSource::METRICS,
                                        "query1".to_string(),
                                        "sum:trace.http.request.errors{*}.as_count()".to_string(),
                                    ),
                                ),
                            ),
                            MonitorFormulaAndFunctionQueryDefinition::MonitorFormulaAndFunctionMetricQueryDefinition(
                                Box::new(
                                    MonitorFormulaAndFunctionMetricQueryDefinition::new(
                                        MonitorFormulaAndFunctionMetricQueryDataSource::METRICS,
                                        "query2".to_string(),
                                        "sum:trace.http.request.hits{*}.as_count()".to_string(),
                                    ),
                                ),
                            )
                        ],
                    ),
            )
            .tags(vec!["test:examplemonitor".to_string(), "env:ci".to_string()]);
    let configuration = datadog::Configuration::new();
    let api = MonitorsAPI::with_config(configuration);
    let resp = api.create_monitor(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
