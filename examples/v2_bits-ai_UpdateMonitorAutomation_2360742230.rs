// Enable automatic investigations for a monitor
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_bits_ai::BitsAIAPI;
use datadog_api_client::datadogV2::model::MonitorAutomationAttributes;
use datadog_api_client::datadogV2::model::MonitorAutomationRequest;
use datadog_api_client::datadogV2::model::MonitorAutomationRequestData;
use datadog_api_client::datadogV2::model::MonitorAutomationType;

#[tokio::main]
async fn main() {
    // there is a valid "monitor" in the system
    let monitor_id: i64 = std::env::var("MONITOR_ID").unwrap().parse().unwrap();
    let body = MonitorAutomationRequest::new(MonitorAutomationRequestData::new(
        MonitorAutomationAttributes::new(true),
        MonitorAutomationType::MONITOR_AUTOMATION,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.update_monitor_automation", true);
    let api = BitsAIAPI::with_config(configuration);
    let resp = api
        .update_monitor_automation(monitor_id.clone(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
