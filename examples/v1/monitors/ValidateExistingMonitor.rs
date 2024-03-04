// Validate an existing monitor returns "OK" response
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
    // there is a valid "monitor" in the system
    let monitor_id: i64 = std::env::var("MONITOR_ID").unwrap().parse().unwrap();
    let body =
        Monitor::new(
            r#"logs("service:foo AND type:error").index("main").rollup("count").by("source").last("5m") > 2"#.to_string(),
            MonitorType::LOG_ALERT,
        )
            .message("some message Notify: @hipchat-channel".to_string())
            .name("Example-Monitor".to_string())
            .options(
                MonitorOptions::new()
                    .enable_logs_sample(true)
                    .escalation_message("the situation has escalated".to_string())
                    .evaluation_delay(Some(700))
                    .groupby_simple_monitor(true)
                    .include_tags(true)
                    .locked(false)
                    .new_host_delay(Some(600))
                    .no_data_timeframe(Some(None))
                    .notification_preset_name(MonitorOptionsNotificationPresets::HIDE_HANDLES)
                    .notify_audit(false)
                    .notify_no_data(false)
                    .on_missing_data(OnMissingDataOption::SHOW_AND_NOTIFY_NO_DATA)
                    .renotify_interval(Some(60))
                    .require_full_window(true)
                    .thresholds(MonitorThresholds::new().critical(2).warning(Some(1)))
                    .timeout_h(Some(24)),
            )
            .priority(Some(3))
            .tags(vec!["test:examplemonitor".to_string(), "env:ci".to_string()]);
    let configuration = Configuration::new();
    let api = MonitorsAPI::with_config(configuration);
    let resp = api.validate_existing_monitor(body).await;
    if let Ok(Some(value)) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
