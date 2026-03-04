// Create a monitor with aggregate augmented query variables returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV1::api_monitors::MonitorsAPI;
use datadog_api_client::datadogV1::model::Monitor;
use datadog_api_client::datadogV1::model::MonitorFormulaAndFunctionAggregateAugmentQuery;
use datadog_api_client::datadogV1::model::MonitorFormulaAndFunctionAggregateAugmentedDataSource;
use datadog_api_client::datadogV1::model::MonitorFormulaAndFunctionAggregateAugmentedQueryDefinition;
use datadog_api_client::datadogV1::model::MonitorFormulaAndFunctionAggregateBaseQuery;
use datadog_api_client::datadogV1::model::MonitorFormulaAndFunctionAggregateQueryJoinCondition;
use datadog_api_client::datadogV1::model::MonitorFormulaAndFunctionAggregateQueryJoinType;
use datadog_api_client::datadogV1::model::MonitorFormulaAndFunctionEventAggregation;
use datadog_api_client::datadogV1::model::MonitorFormulaAndFunctionEventQueryDefinitionCompute;
use datadog_api_client::datadogV1::model::MonitorFormulaAndFunctionEventQueryGroupBy;
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
        Monitor::new(r#"formula("query1").rollup("sum").last("5m") > 124"#.to_string(), MonitorType::QUERY_ALERT)
            .message("test message".to_string())
            .name("Example-Monitor".to_string())
            .options(
                MonitorOptions::new()
                    .thresholds(MonitorThresholds::new().critical(124.0 as f64))
                    .variables(
                        vec![
                            MonitorFormulaAndFunctionQueryDefinition
                            ::MonitorFormulaAndFunctionAggregateAugmentedQueryDefinition(
                                Box::new(
                                    MonitorFormulaAndFunctionAggregateAugmentedQueryDefinition::new(
                                        MonitorFormulaAndFunctionAggregateAugmentQuery
                                        ::MonitorFormulaAndFunctionReferenceTableQueryDefinition(
                                            Box::new(
                                                MonitorFormulaAndFunctionReferenceTableQueryDefinition::new(
                                                    MonitorFormulaAndFunctionReferenceTableDataSource::REFERENCE_TABLE,
                                                    "test_table".to_string(),
                                                )
                                                    .columns(
                                                        vec![
                                                            MonitorFormulaAndFunctionReferenceTableColumn::new(
                                                                "org_id".to_string(),
                                                            ),
                                                            MonitorFormulaAndFunctionReferenceTableColumn::new(
                                                                "name".to_string(),
                                                            )
                                                        ],
                                                    )
                                                    .name("filter_query".to_string()),
                                            ),
                                        ),
                                        MonitorFormulaAndFunctionAggregateBaseQuery
                                        ::MonitorFormulaAndFunctionMetricsQueryDefinition(
                                            Box::new(
                                                MonitorFormulaAndFunctionMetricsQueryDefinition::new(
                                                    MonitorFormulaAndFunctionMetricsDataSource::METRICS,
                                                    "avg:dd{*} by {org_id}.as_count()".to_string(),
                                                ).name("query1".to_string()),
                                            ),
                                        ),
                                        vec![
                                            MonitorFormulaAndFunctionEventQueryDefinitionCompute::new(
                                                MonitorFormulaAndFunctionEventAggregation::MAX,
                                            ).name("compute_result".to_string())
                                        ],
                                        MonitorFormulaAndFunctionAggregateAugmentedDataSource
                                        ::AGGREGATE_AUGMENTED_QUERY,
                                        vec![
                                            MonitorFormulaAndFunctionEventQueryGroupBy::new("org_id".to_string()),
                                            MonitorFormulaAndFunctionEventQueryGroupBy::new("name".to_string())
                                        ],
                                        MonitorFormulaAndFunctionAggregateQueryJoinCondition::new(
                                            "org_id".to_string(),
                                            "org_id".to_string(),
                                            MonitorFormulaAndFunctionAggregateQueryJoinType::INNER,
                                        ),
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
