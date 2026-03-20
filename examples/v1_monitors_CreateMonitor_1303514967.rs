// Create a Cost Monitor returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV1::api_monitors::MonitorsAPI;
use datadog_api_client::datadogV1::model::Monitor;
use datadog_api_client::datadogV1::model::MonitorFormulaAndFunctionCostAggregator;
use datadog_api_client::datadogV1::model::MonitorFormulaAndFunctionCostDataSource;
use datadog_api_client::datadogV1::model::MonitorFormulaAndFunctionCostQueryDefinition;
use datadog_api_client::datadogV1::model::MonitorFormulaAndFunctionQueryDefinition;
use datadog_api_client::datadogV1::model::MonitorOptions;
use datadog_api_client::datadogV1::model::MonitorThresholds;
use datadog_api_client::datadogV1::model::MonitorType;

#[tokio::main]
async fn main() {
    let body =
        Monitor::new(
            r#"formula("exclude_null(query1)").last("7d").anomaly(direction="above", threshold=10) >= 5"#.to_string(),
            MonitorType::COST_ALERT,
        )
            .message("some message Notify: @hipchat-channel".to_string())
            .name("Example Monitor".to_string())
            .options(
                MonitorOptions::new()
                    .include_tags(true)
                    .thresholds(MonitorThresholds::new().critical(5.0 as f64).warning(Some(3.0 as f64)))
                    .variables(
                        vec![
                            MonitorFormulaAndFunctionQueryDefinition::MonitorFormulaAndFunctionCostQueryDefinition(
                                Box::new(
                                    MonitorFormulaAndFunctionCostQueryDefinition::new(
                                        MonitorFormulaAndFunctionCostDataSource::CLOUD_COST,
                                        "query1".to_string(),
                                        "sum:aws.cost.net.amortized.shared.resources.allocated{aws_product IN (amplify ,athena, backup, bedrock ) } by {aws_product}.rollup(sum, 86400)".to_string(),
                                    ).aggregator(MonitorFormulaAndFunctionCostAggregator::SUM),
                                ),
                            )
                        ],
                    ),
            )
            .priority(Some(3))
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
