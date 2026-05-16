// Trigger a Bits AI SRE investigation returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_bits_aisre::BitsAISREAPI;
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
        TriggerInvestigationRequestDataAttributes::new(TriggerAttributes::new(
            MonitorAlertTriggerAttributes::new(
                "1234567890123456789".to_string(),
                1700000000000,
                12345678,
            ),
            TriggerType::MONITOR_ALERT_TRIGGER,
        )),
        TriggerInvestigationRequestType::TRIGGER_INVESTIGATION_REQUEST,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.TriggerInvestigation", true);
    let api = BitsAISREAPI::with_config(configuration);
    let resp = api.trigger_investigation(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
