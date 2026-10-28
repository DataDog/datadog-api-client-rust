// Create an incident responder returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_incidents::IncidentsAPI;
use datadog_api_client::datadogV2::model::IncidentResponderDataRequest;
use datadog_api_client::datadogV2::model::IncidentResponderRelationshipsRequest;
use datadog_api_client::datadogV2::model::IncidentResponderRequest;
use datadog_api_client::datadogV2::model::IncidentResponderType;
use datadog_api_client::datadogV2::model::IncidentResponderUserRelationship;
use datadog_api_client::datadogV2::model::IncidentResponderUserRelationshipData;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body = IncidentResponderRequest::new(IncidentResponderDataRequest::new(
        IncidentResponderRelationshipsRequest::new(IncidentResponderUserRelationship::new(
            IncidentResponderUserRelationshipData::new(
                Uuid::parse_str("00000000-0000-0000-0000-000000000000").expect("invalid UUID"),
                "users".to_string(),
            ),
        )),
        IncidentResponderType::INCIDENT_RESPONDERS,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.create_incident_responder", true);
    let api = IncidentsAPI::with_config(configuration);
    let resp = api
        .create_incident_responder("incident_id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
