// Create a monitor with aggregate filtered query variables returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV1::api_monitors::MonitorsAPI;
use datadog_api_client::datadogV1::model::Monitor;
use datadog_api_client::datadogV1::model::MonitorFormulaAndFunctionAggregateBaseQuery;
use datadog_api_client::datadogV1::model::MonitorFormulaAndFunctionAggregateFilterQuery;
use datadog_api_client::datadogV1::model::MonitorFormulaAndFunctionAggregateFilteredDataSource;
use datadog_api_client::datadogV1::model::MonitorFormulaAndFunctionAggregateFilteredQueryDefinition;
use datadog_api_client::datadogV1::model::MonitorFormulaAndFunctionAggregateQueryFilter;
use datadog_api_client::datadogV1::model::MonitorFormulaAndFunctionMetricsDataSource;
use datadog_api_client::datadogV1::model::MonitorFormulaAndFunctionMetricsQueryDefinition;
use datadog_api_client::datadogV1::model::MonitorFormulaAndFunctionQueryDefinition;
use datadog_api_client::datadogV1::model::MonitorFormulaAndFunctionReferenceTableColumn;
use datadog_api_client::datadogV1::model::MonitorFormulaAndFunctionReferenceTableDataSource;
use datadog_api_client::datadogV1::model::MonitorFormulaAndFunctionReferenceTableQueryDefinition;
use datadog_api_client::datadogV1::model::MonitorOptions;
use datadog_api_client::datadogV1::model::MonitorThresholds;
use datadog_api_client::datadogV1::model::MonitorType;

#[tokio::main]
async fn main() {
    let body =
        Monitor::new(r#"formula("query1").rollup("sum").last("5m") > 100"#.to_string(), MonitorType::QUERY_ALERT)
            .message("test message".to_string())
            .name("Example-Monitor".to_string())
            .options(
                MonitorOptions::new()
                    .thresholds(MonitorThresholds::new().critical(100.0 as f64))
                    .variables(
                        vec![
                            MonitorFormulaAndFunctionQueryDefinition
                            ::MonitorFormulaAndFunctionAggregateFilteredQueryDefinition(
                                Box::new(
                                    MonitorFormulaAndFunctionAggregateFilteredQueryDefinition::new(
                                        MonitorFormulaAndFunctionAggregateBaseQuery
                                        ::MonitorFormulaAndFunctionMetricsQueryDefinition(
                                            Box::new(
                                                MonitorFormulaAndFunctionMetricsQueryDefinition::new(
                                                    MonitorFormulaAndFunctionMetricsDataSource::METRICS,
                                                    "max:container.cpu.usage{*} by {kube_cluster_name}.rollup(max)".to_string(),
                                                ).name("query1".to_string()),
                                            ),
                                        ),
                                        MonitorFormulaAndFunctionAggregateFilteredDataSource::AGGREGATE_FILTERED_QUERY,
                                        MonitorFormulaAndFunctionAggregateFilterQuery
                                        ::MonitorFormulaAndFunctionReferenceTableQueryDefinition(
                                            Box::new(
                                                MonitorFormulaAndFunctionReferenceTableQueryDefinition::new(
                                                    MonitorFormulaAndFunctionReferenceTableDataSource::REFERENCE_TABLE,
                                                    "test_table".to_string(),
                                                )
                                                    .columns(
                                                        vec![
                                                            MonitorFormulaAndFunctionReferenceTableColumn::new(
                                                                "cluster_name".to_string(),
                                                            )
                                                        ],
                                                    )
                                                    .name("filter_query".to_string()),
                                            ),
                                        ),
                                        vec![
                                            MonitorFormulaAndFunctionAggregateQueryFilter::new(
                                                "kube_cluster_name".to_string(),
                                                "cluster_name".to_string(),
                                            )
                                        ],
                                    ).name("query1".to_string()),
                                ),
                            )
                        ],
                    ),
            );
    let configuration = datadog::Configuration::new();
    let api = MonitorsAPI::with_config(configuration);
    let resp = api.create_monitor(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
