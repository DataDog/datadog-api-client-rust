// Create an incident Zoom meeting returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_incidents::IncidentsAPI;
use datadog_api_client::datadogV2::model::IncidentCreateZoomMeetingData;
use datadog_api_client::datadogV2::model::IncidentCreateZoomMeetingDataAttributes;
use datadog_api_client::datadogV2::model::IncidentCreateZoomMeetingRequest;
use datadog_api_client::datadogV2::model::IncidentZoomIntegrationType;

#[tokio::main]
async fn main() {
    let body = IncidentCreateZoomMeetingRequest::new(IncidentCreateZoomMeetingData::new(
        IncidentCreateZoomMeetingDataAttributes::new("Incident INC-123 War Room".to_string()),
        IncidentZoomIntegrationType::INCIDENT_INTEGRATIONS,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateIncidentZoomMeeting", true);
    let api = IncidentsAPI::with_config(configuration);
    let resp = api
        .create_incident_zoom_meeting("incident_id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
