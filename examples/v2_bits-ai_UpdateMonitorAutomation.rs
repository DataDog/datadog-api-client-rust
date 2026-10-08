// Update monitor automatic investigation settings returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_bits_ai::BitsAIAPI;
use datadog_api_client::datadogV2::model::MonitorAutomationAttributes;
use datadog_api_client::datadogV2::model::MonitorAutomationRequest;
use datadog_api_client::datadogV2::model::MonitorAutomationRequestData;
use datadog_api_client::datadogV2::model::MonitorAutomationType;

#[tokio::main]
async fn main() {
    let body = MonitorAutomationRequest::new(MonitorAutomationRequestData::new(
        MonitorAutomationAttributes::new(true),
        MonitorAutomationType::MONITOR_AUTOMATION,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.update_monitor_automation", true);
    let api = BitsAIAPI::with_config(configuration);
    let resp = api
        .update_monitor_automation(9223372036854775807, body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
