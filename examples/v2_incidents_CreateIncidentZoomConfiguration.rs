// Create an incident Zoom configuration returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_incidents::CreateIncidentZoomConfigurationOptionalParams;
use datadog_api_client::datadogV2::api_incidents::IncidentsAPI;
use datadog_api_client::datadogV2::model::IncidentZoomConfigurationDataAttributesRequest;
use datadog_api_client::datadogV2::model::IncidentZoomConfigurationDataRequest;
use datadog_api_client::datadogV2::model::IncidentZoomConfigurationRequest;
use datadog_api_client::datadogV2::model::IncidentZoomConfigurationType;

#[tokio::main]
async fn main() {
    let body = IncidentZoomConfigurationRequest::new(IncidentZoomConfigurationDataRequest::new(
        IncidentZoomConfigurationDataAttributesRequest::new()
            .manual_meeting_creation(false)
            .meeting_chat_timeline_sync(false)
            .post_meeting_summary(true),
        IncidentZoomConfigurationType::ZOOM_CONFIGURATIONS,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateIncidentZoomConfiguration", true);
    let api = IncidentsAPI::with_config(configuration);
    let resp = api
        .create_incident_zoom_configuration(
            body,
            CreateIncidentZoomConfigurationOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
