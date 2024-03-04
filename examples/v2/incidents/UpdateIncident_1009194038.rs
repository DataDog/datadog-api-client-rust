// Remove commander from an incident returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_incidents::IncidentsAPI;
use datadog_api_client::datadogV2::model::*;
use std::ops::Add;
use std::time::{
    Duration,
    SystemTime,
    UNIX_EPOCH,
};

#[tokio::main]
async fn main() {
    // there is a valid "incident" in the system
    let incident_data_id = std::env::var("INCIDENT_DATA_ID").unwrap();
    let body =
        IncidentUpdateRequest::new(
            IncidentUpdateData::new(
                incident_data_id,
                IncidentType::INCIDENTS,
            ).relationships(
                IncidentUpdateRelationships
                ::new().commander_user(Some(NullableRelationshipToUser::new(Some(None)))),
            ),
        );
    let configuration = Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdateIncident", true);
    let api = IncidentsAPI::with_config(configuration);
    let resp = api.update_incident(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
