// Update an existing incident integration metadata returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_incidents::IncidentsAPI;
use datadog_api_client::datadogV2::model::*;

#[tokio::main]
async fn main() {
    // there is a valid "incident" in the system
    let incident_data_id = std::env::var("INCIDENT_DATA_ID").unwrap();

    // the "incident" has an "incident_integration_metadata"
    let incident_integration_metadata_data_id = std::env::var("INCIDENT_INTEGRATION_METADATA_DATA_ID").unwrap();
    let body =
        IncidentIntegrationMetadataPatchRequest::new(
            IncidentIntegrationMetadataPatchData::new(
                IncidentIntegrationMetadataAttributes::new(
                    1,
                    IncidentIntegrationMetadataMetadata::SlackIntegrationMetadata(
                        Box::new(
                            SlackIntegrationMetadata::new(
                                vec![
                                    SlackIntegrationMetadataChannelItem::new(
                                        "C0123456789".to_string(),
                                        "#updated-channel-name".to_string(),
                                        "https://slack.com/app_redirect?channel=C0123456789&team=T01234567".to_string(),
                                    ).team_id("T01234567".to_string())
                                ],
                            ),
                        ),
                    ),
                ).incident_id(incident_data_id),
                IncidentIntegrationMetadataType::INCIDENT_INTEGRATIONS,
            ),
        );
    let configuration = Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdateIncidentIntegration", true);
    let api = IncidentsAPI::with_config(configuration);
    let resp = api.update_incident_integration(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
