// Create an incident attachment returns "OK" response
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
        IncidentAttachmentUpdateRequest::new(
            vec![
                IncidentAttachmentUpdateData::new(
                    IncidentAttachmentType::INCIDENT_ATTACHMENTS,
                ).attributes(
                    IncidentAttachmentUpdateAttributes::IncidentAttachmentLinkAttributes(
                        Box::new(
                            IncidentAttachmentLinkAttributes::new(
                                IncidentAttachmentLinkAttributesAttachmentObject::new(
                                    "https://www.example.com/doc".to_string(),
                                    "Example-Incident".to_string(),
                                ),
                                IncidentAttachmentLinkAttachmentType::LINK,
                            ),
                        ),
                    ),
                )
            ],
        );
    let configuration = Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdateIncidentAttachments", true);
    let api = IncidentsAPI::with_config(configuration);
    let resp = api.update_incident_attachments(body).await;
    if let Ok(Some(value)) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
