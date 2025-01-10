// Delete an incident type returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_incidents::IncidentsAPI;

#[tokio::main]
async fn main() {
    // there is a valid "incident_type" in the system
    let incident_type_data_id = std::env::var("INCIDENT_TYPE_DATA_ID").unwrap();

    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.DeleteIncidentType", true);
    let api = IncidentsAPI::with_config(configuration);
    let resp = api
        .delete_incident_type(incident_type_data_id.clone())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
