// Get a list of an incident's todos returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_incidents::*;

#[tokio::main]
async fn main() {
    // there is a valid "incident" in the system
    let incident_data_id = std::env::var("INCIDENT_DATA_ID").unwrap();
    let mut configuration = Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ListIncidentTodos", true);
    let api = IncidentsAPI::with_config(configuration);
    let resp = api.list_incident_todos(incident_data_id.clone()).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}