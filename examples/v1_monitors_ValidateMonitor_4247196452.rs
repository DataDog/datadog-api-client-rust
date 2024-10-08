// Validate a multi-alert monitor returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV1::api_monitors::MonitorsAPI;
use datadog_api_client::datadogV1::model::Monitor;
use datadog_api_client::datadogV1::model::MonitorOptions;
use datadog_api_client::datadogV1::model::MonitorThresholds;
use datadog_api_client::datadogV1::model::MonitorType;
use datadog_api_client::datadogV1::model::OnMissingDataOption;

#[tokio::main]
async fn main() {
    let body =
        Monitor::new(
            r#"logs("service:foo AND type:error").index("main").rollup("count").by("source,status").last("5m") > 2"#.to_string(),
            MonitorType::LOG_ALERT,
        )
            .message("some message Notify: @hipchat-channel".to_string())
            .name("Example-Monitor".to_string())
            .options(
                MonitorOptions::new()
                    .enable_logs_sample(true)
                    .escalation_message("the situation has escalated".to_string())
                    .evaluation_delay(Some(700))
                    .group_retention_duration("2d".to_string())
                    .include_tags(true)
                    .locked(false)
                    .new_host_delay(Some(600))
                    .no_data_timeframe(None)
                    .notify_audit(false)
                    .notify_by(vec!["status".to_string()])
                    .notify_no_data(false)
                    .on_missing_data(OnMissingDataOption::SHOW_AND_NOTIFY_NO_DATA)
                    .renotify_interval(Some(60))
                    .require_full_window(true)
                    .thresholds(MonitorThresholds::new().critical(2.0 as f64).warning(Some(1.0 as f64)))
                    .timeout_h(Some(24)),
            )
            .priority(Some(3))
            .tags(vec!["test:examplemonitor".to_string(), "env:ci".to_string()]);
    let configuration = datadog::Configuration::new();
    let api = MonitorsAPI::with_config(configuration);
    let resp = api.validate_monitor(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
