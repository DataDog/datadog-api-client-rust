// Create a Data Quality monitor returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV1::api_monitors::MonitorsAPI;
use datadog_api_client::datadogV1::model::Monitor;
use datadog_api_client::datadogV1::model::MonitorFormulaAndFunctionDataQualityDataSource;
use datadog_api_client::datadogV1::model::MonitorFormulaAndFunctionDataQualityQueryDefinition;
use datadog_api_client::datadogV1::model::MonitorFormulaAndFunctionQueryDefinition;
use datadog_api_client::datadogV1::model::MonitorOptions;
use datadog_api_client::datadogV1::model::MonitorThresholds;
use datadog_api_client::datadogV1::model::MonitorType;

#[tokio::main]
async fn main() {
    let body =
        Monitor::new(r#"formula("query1").last("5m") > 100"#.to_string(), MonitorType::DATA_QUALITY_ALERT)
            .message("Data quality alert triggered".to_string())
            .name("Example-Monitor".to_string())
            .options(
                MonitorOptions::new()
                    .thresholds(MonitorThresholds::new().critical(100.0 as f64))
                    .variables(
                        vec![
                            MonitorFormulaAndFunctionQueryDefinition
                            ::MonitorFormulaAndFunctionDataQualityQueryDefinition(
                                Box::new(
                                    MonitorFormulaAndFunctionDataQualityQueryDefinition::new(
                                        MonitorFormulaAndFunctionDataQualityDataSource::DATA_QUALITY_METRICS,
                                        r#"search for column where `database:production AND table:users`"#.to_string(),
                                        "row_count".to_string(),
                                        "query1".to_string(),
                                    ).group_by(vec!["entity_id".to_string()]),
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
