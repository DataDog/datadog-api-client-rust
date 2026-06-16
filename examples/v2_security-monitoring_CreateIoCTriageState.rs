// Create or update an indicator triage state returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::IoCTriageState;
use datadog_api_client::datadogV2::model::IoCTriageWriteRequest;
use datadog_api_client::datadogV2::model::IoCTriageWriteRequestAttributes;
use datadog_api_client::datadogV2::model::IoCTriageWriteRequestData;

#[tokio::main]
async fn main() {
    let body = IoCTriageWriteRequest::new(IoCTriageWriteRequestData::new(
        IoCTriageWriteRequestAttributes::new("192.0.2.1".to_string(), IoCTriageState::REVIEWED),
        "ioc_triage_state".to_string(),
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateIoCTriageState", true);
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api.create_io_c_triage_state(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
