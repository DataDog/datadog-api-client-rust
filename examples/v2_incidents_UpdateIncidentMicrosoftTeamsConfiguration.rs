// Update an incident Microsoft Teams configuration returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_incidents::IncidentsAPI;
use datadog_api_client::datadogV2::api_incidents::UpdateIncidentMicrosoftTeamsConfigurationOptionalParams;
use datadog_api_client::datadogV2::model::IncidentMicrosoftTeamsConfigurationDataAttributesRequest;
use datadog_api_client::datadogV2::model::IncidentMicrosoftTeamsConfigurationDataRequest;
use datadog_api_client::datadogV2::model::IncidentMicrosoftTeamsConfigurationRequest;
use datadog_api_client::datadogV2::model::IncidentMicrosoftTeamsConfigurationType;
use uuid::Uuid;

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
        .set_unstable_operation_enabled("v2.UpdateIncidentMicrosoftTeamsConfiguration", true);
    let api = IncidentsAPI::with_config(configuration);
    let resp = api
        .update_incident_microsoft_teams_configuration(
            Uuid::parse_str("00000000-0000-0000-0000-000000000000").expect("invalid UUID"),
            body,
            UpdateIncidentMicrosoftTeamsConfigurationOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
