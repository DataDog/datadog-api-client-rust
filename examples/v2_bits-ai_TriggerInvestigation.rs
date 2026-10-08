// Trigger a Bits AI investigation returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_bits_ai::BitsAIAPI;
use datadog_api_client::datadogV2::model::MonitorAlertTriggerAttributes;
use datadog_api_client::datadogV2::model::TriggerAttributes;
use datadog_api_client::datadogV2::model::TriggerInvestigationRequest;
use datadog_api_client::datadogV2::model::TriggerInvestigationRequestData;
use datadog_api_client::datadogV2::model::TriggerInvestigationRequestDataAttributes;
use datadog_api_client::datadogV2::model::TriggerInvestigationRequestType;
use datadog_api_client::datadogV2::model::TriggerType;

#[tokio::main]
async fn main() {
    let body = TriggerInvestigationRequest::new(TriggerInvestigationRequestData::new(
        TriggerInvestigationRequestDataAttributes::new(
            TriggerAttributes::new()
                .monitor_alert_trigger(MonitorAlertTriggerAttributes::new(
                    "1234567890123456789".to_string(),
                    1700000000000,
                    12345678,
                ))
                .type_(TriggerType::MONITOR_ALERT_TRIGGER),
        ),
        TriggerInvestigationRequestType::TRIGGER_INVESTIGATION_REQUEST,
    ));
    let configuration = datadog::Configuration::new();
    let api = BitsAIAPI::with_config(configuration);
    let resp = api.trigger_investigation(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
