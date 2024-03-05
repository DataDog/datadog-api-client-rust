// Edit a monitor returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_monitors::MonitorsAPI;
use datadog_api_client::datadogV1::model::*;

#[tokio::main]
async fn main() {
    // there is a valid "monitor" in the system
    let monitor_id: i64 = std::env::var("MONITOR_ID").unwrap().parse().unwrap();
    let body =
        MonitorUpdateRequest::new()
            .name("My monitor-updated".to_string())
            .options(
                MonitorOptions::new()
                    .evaluation_delay(Some(None))
                    .new_group_delay(Some(600))
                    .new_host_delay(Some(None))
                    .renotify_interval(Some(None))
                    .thresholds(MonitorThresholds::new().critical(2).warning(Some(None)))
                    .timeout_h(Some(None)),
            );
    let configuration = Configuration::new();
    let api = MonitorsAPI::with_config(configuration);
    let resp = api.update_monitor(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
