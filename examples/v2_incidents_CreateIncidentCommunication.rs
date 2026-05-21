// Create an incident communication returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_incidents::IncidentsAPI;
use datadog_api_client::datadogV2::model::IncidentCommunicationContent;
use datadog_api_client::datadogV2::model::IncidentCommunicationContentHandle;
use datadog_api_client::datadogV2::model::IncidentCommunicationDataAttributesRequest;
use datadog_api_client::datadogV2::model::IncidentCommunicationDataRequest;
use datadog_api_client::datadogV2::model::IncidentCommunicationKind;
use datadog_api_client::datadogV2::model::IncidentCommunicationRequest;
use datadog_api_client::datadogV2::model::IncidentCommunicationType;

#[tokio::main]
async fn main() {
    let body = IncidentCommunicationRequest::new(IncidentCommunicationDataRequest::new(
        IncidentCommunicationDataAttributesRequest::new(
            IncidentCommunicationKind::MANUAL,
            IncidentCommunicationContent::new(
                vec![IncidentCommunicationContentHandle::new(
                    "@slack-incidents-channel".to_string(),
                )
                .created_at("2024-01-01T00:00:00.000Z".to_string())
                .display_name("#incidents-channel".to_string())],
                "Incident update for INC-123.".to_string(),
            )
            .grouping_key("update-1".to_string())
            .status(0)
            .subject("Incident INC-123: Update".to_string()),
        ),
        IncidentCommunicationType::COMMUNICATION,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateIncidentCommunication", true);
    let api = IncidentsAPI::with_config(configuration);
    let resp = api
        .create_incident_communication("incident_id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
