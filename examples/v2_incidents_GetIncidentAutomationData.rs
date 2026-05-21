// Get incident automation data returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_incidents::GetIncidentAutomationDataOptionalParams;
use datadog_api_client::datadogV2::api_incidents::IncidentsAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.GetIncidentAutomationData", true);
    let api = IncidentsAPI::with_config(configuration);
    let resp = api
        .get_incident_automation_data(
            "incident_id".to_string(),
            "key".to_string(),
            GetIncidentAutomationDataOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
