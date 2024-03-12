// Delete an incident integration metadata returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_incidents::*;

#[tokio::main]
async fn main() {
    // there is a valid "incident" in the system
    let incident_data_id = std::env::var("INCIDENT_DATA_ID").unwrap();

    // the "incident" has an "incident_integration_metadata"
    let incident_integration_metadata_data_id =
        std::env::var("INCIDENT_INTEGRATION_METADATA_DATA_ID").unwrap();
    let mut configuration = Configuration::new();
    configuration.set_unstable_operation_enabled("v2.DeleteIncidentIntegration", true);
    let api = IncidentsAPI::with_config(configuration);
    let resp = api
        .delete_incident_integration(
            incident_data_id.clone(),
            incident_integration_metadata_data_id.clone(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
