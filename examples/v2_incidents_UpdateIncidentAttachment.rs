// Update incident attachment returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_incidents::IncidentsAPI;
use datadog_api_client::datadogV2::api_incidents::UpdateIncidentAttachmentOptionalParams;
use datadog_api_client::datadogV2::model::IncidentAttachmentType;
use datadog_api_client::datadogV2::model::PatchAttachmentRequest;
use datadog_api_client::datadogV2::model::PatchAttachmentRequestData;
use datadog_api_client::datadogV2::model::PatchAttachmentRequestDataAttributes;
use datadog_api_client::datadogV2::model::PatchAttachmentRequestDataAttributesAttachment;

#[tokio::main]
async fn main() {
    // there is a valid "incident" in the system
    let incident_data_id = std::env::var("INCIDENT_DATA_ID").unwrap();

    // there is a valid "incident_attachment" in the system
    let incident_attachment_data_id = std::env::var("INCIDENT_ATTACHMENT_DATA_ID").unwrap();
    let body = PatchAttachmentRequest::new().data(
        PatchAttachmentRequestData::new(IncidentAttachmentType::INCIDENT_ATTACHMENTS)
            .attributes(
                PatchAttachmentRequestDataAttributes::new().attachment(
                    PatchAttachmentRequestDataAttributesAttachment::new()
                        .document_url(
                            "https://app.datadoghq.com/notebook/124/Example-Incident".to_string(),
                        )
                        .title("Example-Incident".to_string()),
                ),
            )
            .id(incident_attachment_data_id.clone()),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdateIncidentAttachment", true);
    let api = IncidentsAPI::with_config(configuration);
    let resp = api
        .update_incident_attachment(
            incident_data_id.clone(),
            incident_attachment_data_id.clone(),
            body,
            UpdateIncidentAttachmentOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
