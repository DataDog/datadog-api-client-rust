// Get all monitor notification rules returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_monitors::GetMonitorNotificationRulesOptionalParams;
use datadog_api_client::datadogV2::api_monitors::MonitorsAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = MonitorsAPI::with_config(configuration);
    let resp = api
        .get_monitor_notification_rules(GetMonitorNotificationRulesOptionalParams::default())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
