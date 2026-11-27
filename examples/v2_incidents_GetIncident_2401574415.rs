// Get the details of an incident includes responders with modification timestamps
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_incidents::GetIncidentOptionalParams;
use datadog_api_client::datadogV2::api_incidents::IncidentsAPI;
use datadog_api_client::datadogV2::model::IncidentRelatedObject;

#[tokio::main]
async fn main() {
    // there is a valid "incident_with_commander" in the system
    let incident_with_commander_data_id = std::env::var("INCIDENT_WITH_COMMANDER_DATA_ID").unwrap();
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.create_incident", true);
    configuration.set_unstable_operation_enabled("v2.get_incident", true);
    let api = IncidentsAPI::with_config(configuration);
    let resp = api
        .get_incident(
            incident_with_commander_data_id.clone(),
            GetIncidentOptionalParams::default().include(vec![IncidentRelatedObject::RESPONDERS]),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
