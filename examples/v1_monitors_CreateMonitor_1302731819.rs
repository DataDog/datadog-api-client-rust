// Create a Data Jobs monitor with the aastra query dialect returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV1::api_monitors::MonitorsAPI;
use datadog_api_client::datadogV1::model::Monitor;
use datadog_api_client::datadogV1::model::MonitorFormulaAndFunctionDataJobsMonitorOptions;
use datadog_api_client::datadogV1::model::MonitorFormulaAndFunctionDataJobsQueryDefinition;
use datadog_api_client::datadogV1::model::MonitorFormulaAndFunctionQueryDefinition;
use datadog_api_client::datadogV1::model::MonitorOptions;
use datadog_api_client::datadogV1::model::MonitorThresholds;
use datadog_api_client::datadogV1::model::MonitorType;

#[tokio::main]
async fn main() {
    let body =
        Monitor::new(
            r#"formula("failed_runs(run_query)").by(name,workspace_name,entity_id).last(1d) > 0"#.to_string(),
            MonitorType::DATA_JOBS_ALERT,
        )
            .message("Data jobs alert triggered".to_string())
            .name("Example-Monitor".to_string())
            .options(
                MonitorOptions::new()
                    .thresholds(MonitorThresholds::new().critical(0.0 as f64))
                    .variables(
                        vec![
                            MonitorFormulaAndFunctionQueryDefinition::MonitorFormulaAndFunctionDataJobsQueryDefinition(
                                Box::new(
                                    MonitorFormulaAndFunctionDataJobsQueryDefinition::new(
                                        "databricks.job".to_string(),
                                        "search for databricks_job".to_string(),
                                        "run_query".to_string(),
                                        "aastra".to_string(),
                                    )
                                        .monitor_options(
                                            MonitorFormulaAndFunctionDataJobsMonitorOptions
                                            ::new().alert_on_every_failure(true),
                                        )
                                        .run_filter("env:prod".to_string()),
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
