// Create an incident Microsoft Teams configuration returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_incidents::CreateIncidentMicrosoftTeamsConfigurationOptionalParams;
use datadog_api_client::datadogV2::api_incidents::IncidentsAPI;
use datadog_api_client::datadogV2::model::IncidentMicrosoftTeamsConfigurationDataAttributesRequest;
use datadog_api_client::datadogV2::model::IncidentMicrosoftTeamsConfigurationDataRequest;
use datadog_api_client::datadogV2::model::IncidentMicrosoftTeamsConfigurationRequest;
use datadog_api_client::datadogV2::model::IncidentMicrosoftTeamsConfigurationType;

#[tokio::main]
async fn main() {
    let body = IncidentMicrosoftTeamsConfigurationRequest::new(
        IncidentMicrosoftTeamsConfigurationDataRequest::new(
            IncidentMicrosoftTeamsConfigurationDataAttributesRequest::new()
                .manual_meeting_creation(false)
                .post_meeting_summary(true),
            IncidentMicrosoftTeamsConfigurationType::MICROSOFT_TEAMS_CONFIGURATIONS,
        ),
    );
    let mut configuration = datadog::Configuration::new();
    configuration
        .set_unstable_operation_enabled("v2.CreateIncidentMicrosoftTeamsConfiguration", true);
    let api = IncidentsAPI::with_config(configuration);
    let resp = api
        .create_incident_microsoft_teams_configuration(
            body,
            CreateIncidentMicrosoftTeamsConfigurationOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
