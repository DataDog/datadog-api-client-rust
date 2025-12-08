// Create incident attachment returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_incidents::CreateIncidentAttachmentOptionalParams;
use datadog_api_client::datadogV2::api_incidents::IncidentsAPI;
use datadog_api_client::datadogV2::model::AttachmentDataAttributesAttachmentType;
use datadog_api_client::datadogV2::model::CreateAttachmentRequest;
use datadog_api_client::datadogV2::model::CreateAttachmentRequestData;
use datadog_api_client::datadogV2::model::CreateAttachmentRequestDataAttributes;
use datadog_api_client::datadogV2::model::CreateAttachmentRequestDataAttributesAttachment;
use datadog_api_client::datadogV2::model::IncidentAttachmentType;

#[tokio::main]
async fn main() {
    let body = CreateAttachmentRequest::new().data(
        CreateAttachmentRequestData::new(IncidentAttachmentType::INCIDENT_ATTACHMENTS)
            .attributes(
                CreateAttachmentRequestDataAttributes::new()
                    .attachment(
                        CreateAttachmentRequestDataAttributesAttachment::new()
                            .document_url(
                                "https://app.datadoghq.com/notebook/123/Postmortem-IR-123"
                                    .to_string(),
                            )
                            .title("Postmortem-IR-123".to_string()),
                    )
                    .attachment_type(AttachmentDataAttributesAttachmentType::POSTMORTEM),
            )
            .id("00000000-0000-0000-0000-000000000000".to_string()),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateIncidentAttachment", true);
    let api = IncidentsAPI::with_config(configuration);
    let resp = api
        .create_incident_attachment(
            "incident_id".to_string(),
            body,
            CreateIncidentAttachmentOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
