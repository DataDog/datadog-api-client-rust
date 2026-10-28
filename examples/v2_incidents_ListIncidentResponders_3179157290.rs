// List incident responders includes users and role assignments
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_incidents::IncidentsAPI;
use datadog_api_client::datadogV2::api_incidents::ListIncidentRespondersOptionalParams;
use datadog_api_client::datadogV2::model::IncidentResponderRelatedObject;

#[tokio::main]
async fn main() {
    // there is a valid "incident_with_commander" in the system
    let incident_with_commander_data_id = std::env::var("INCIDENT_WITH_COMMANDER_DATA_ID").unwrap();
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.create_incident", true);
    configuration.set_unstable_operation_enabled("v2.list_incident_responders", true);
    let api = IncidentsAPI::with_config(configuration);
    let resp = api
        .list_incident_responders(
            incident_with_commander_data_id.clone(),
            ListIncidentRespondersOptionalParams::default().include(vec![
                IncidentResponderRelatedObject::USERS,
                IncidentResponderRelatedObject::ROLE_ASSIGNMENTS,
            ]),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
