// Delete incident attachment returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_incidents::IncidentsAPI;

#[tokio::main]
async fn main() {
    // there is a valid "incident" in the system
    let incident_data_id = std::env::var("INCIDENT_DATA_ID").unwrap();

    // there is a valid "incident_attachment" in the system
    let incident_attachment_data_id = std::env::var("INCIDENT_ATTACHMENT_DATA_ID").unwrap();
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.DeleteIncidentAttachment", true);
    let api = IncidentsAPI::with_config(configuration);
    let resp = api
        .delete_incident_attachment(
            incident_data_id.clone(),
            incident_attachment_data_id.clone(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
