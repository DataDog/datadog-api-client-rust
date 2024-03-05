// Get details of an incident service returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_incident_services::IncidentServicesAPI;
use datadog_api_client::datadogV2::model::*;

#[tokio::main]
async fn main() {
    // there is a valid "service" in the system
    let service_data_id = std::env::var("SERVICE_DATA_ID").unwrap();
    let configuration = Configuration::new();
    configuration.set_unstable_operation_enabled("v2.GetIncidentService", true);
    let api = IncidentServicesAPI::with_config(configuration);
    let resp = api.get_incident_service().await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
