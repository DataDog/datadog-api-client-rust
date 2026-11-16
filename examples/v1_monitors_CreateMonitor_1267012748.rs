// Create a Data Jobs duration monitor returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV1::api_monitors::MonitorsAPI;
use datadog_api_client::datadogV1::model::Monitor;
use datadog_api_client::datadogV1::model::MonitorFormulaAndFunctionDataQualityDataSource;
use datadog_api_client::datadogV1::model::MonitorFormulaAndFunctionDataQualityDetectionType;
use datadog_api_client::datadogV1::model::MonitorFormulaAndFunctionDataQualityMonitorOptions;
use datadog_api_client::datadogV1::model::MonitorFormulaAndFunctionDataQualityQueryDefinition;
use datadog_api_client::datadogV1::model::MonitorFormulaAndFunctionQueryDefinition;
use datadog_api_client::datadogV1::model::MonitorOptions;
use datadog_api_client::datadogV1::model::MonitorType;

#[tokio::main]
async fn main() {
    let body =
        Monitor::new(
            r#"formula("job_duration(run_query)").detection(anomaly).by(name,workspace_name,entity_id) > 0"#.to_string(),
            MonitorType::DATA_JOBS_ALERT,
        )
            .message("Data jobs duration alert triggered".to_string())
            .name("Example-Monitor".to_string())
            .options(
                MonitorOptions
                ::new().variables(
                    vec![
                        MonitorFormulaAndFunctionQueryDefinition::MonitorFormulaAndFunctionDataQualityQueryDefinition(
                            Box::new(
                                MonitorFormulaAndFunctionDataQualityQueryDefinition::new(
                                    MonitorFormulaAndFunctionDataQualityDataSource::DATA_QUALITY_METRICS,
                                    "search for databricks_job".to_string(),
                                    "duration".to_string(),
                                    "run_query".to_string(),
                                )
                                    .group_by(
                                        vec!["name".to_string(), "workspace_name".to_string(), "entity_id".to_string()],
                                    )
                                    .monitor_options(
                                        MonitorFormulaAndFunctionDataQualityMonitorOptions::new()
                                            .consecutive_runs(2)
                                            .detection_type(
                                                MonitorFormulaAndFunctionDataQualityDetectionType::ANOMALY,
                                            ),
                                    )
                                    .run_filter("env:prod".to_string())
                                    .schema_version("0.0.1".to_string()),
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
