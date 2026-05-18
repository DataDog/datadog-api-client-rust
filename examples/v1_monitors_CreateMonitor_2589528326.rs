// Create a Data Jobs monitor returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV1::api_monitors::MonitorsAPI;
use datadog_api_client::datadogV1::model::Monitor;
use datadog_api_client::datadogV1::model::MonitorFormulaAndFunctionDataJobsQueryDefinition;
use datadog_api_client::datadogV1::model::MonitorFormulaAndFunctionQueryDefinition;
use datadog_api_client::datadogV1::model::MonitorOptions;
use datadog_api_client::datadogV1::model::MonitorThresholds;
use datadog_api_client::datadogV1::model::MonitorType;

#[tokio::main]
async fn main() {
    let body =
        Monitor::new(
            r#"formula("failed_runs(run_query)").by(job_name,workspace_name).last(10d) > 0"#.to_string(),
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
                                        "job_name:*".to_string(),
                                        "run_query".to_string(),
                                        "metric".to_string(),
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
