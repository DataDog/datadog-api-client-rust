// Create an LLM Observability monitor returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV1::api_monitors::MonitorsAPI;
use datadog_api_client::datadogV1::model::Monitor;
use datadog_api_client::datadogV1::model::MonitorOptions;
use datadog_api_client::datadogV1::model::MonitorThresholds;
use datadog_api_client::datadogV1::model::MonitorType;

#[tokio::main]
async fn main() {
    let body = Monitor::new(
        r#"llm-observability("*").rollup("count").last("2h") > 0"#.to_string(),
        MonitorType::LLM_OBSERVABILITY_ALERT,
    )
    .message("LLM observability alert triggered".to_string())
    .name("Example-Monitor".to_string())
    .options(
        MonitorOptions::new()
            .include_tags(true)
            .notify_audit(false)
            .thresholds(MonitorThresholds::new().critical(0.0 as f64)),
    )
    .tags(vec![
        "test:examplemonitor".to_string(),
        "env:ci".to_string(),
    ]);
    let configuration = datadog::Configuration::new();
    let api = MonitorsAPI::with_config(configuration);
    let resp = api.create_monitor(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
