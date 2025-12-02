// Create a monitor with assets returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV1::api_monitors::MonitorsAPI;
use datadog_api_client::datadogV1::model::Monitor;
use datadog_api_client::datadogV1::model::MonitorAsset;
use datadog_api_client::datadogV1::model::MonitorAssetCategory;
use datadog_api_client::datadogV1::model::MonitorAssetResourceType;
use datadog_api_client::datadogV1::model::MonitorOptions;
use datadog_api_client::datadogV1::model::MonitorOptionsSchedulingOptions;
use datadog_api_client::datadogV1::model::MonitorOptionsSchedulingOptionsEvaluationWindow;
use datadog_api_client::datadogV1::model::MonitorThresholds;
use datadog_api_client::datadogV1::model::MonitorType;

#[tokio::main]
async fn main() {
    let body = Monitor::new(
        "avg(current_1mo):avg:system.load.5{*} > 0.5".to_string(),
        MonitorType::METRIC_ALERT,
    )
    .assets(vec![MonitorAsset::new(
        MonitorAssetCategory::RUNBOOK,
        "Monitor Runbook".to_string(),
        "/notebooks/12345".to_string(),
    )
    .resource_key("12345".to_string())
    .resource_type(MonitorAssetResourceType::NOTEBOOK)])
    .message("some message Notify: @hipchat-channel".to_string())
    .name("Example-Monitor".to_string())
    .options(
        MonitorOptions::new()
            .scheduling_options(
                MonitorOptionsSchedulingOptions::new().evaluation_window(
                    MonitorOptionsSchedulingOptionsEvaluationWindow::new()
                        .day_starts("04:00".to_string())
                        .month_starts(1),
                ),
            )
            .thresholds(MonitorThresholds::new().critical(0.5 as f64)),
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
