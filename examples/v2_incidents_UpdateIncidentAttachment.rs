// Update incident attachment returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_incidents::IncidentsAPI;
use datadog_api_client::datadogV2::api_incidents::UpdateIncidentAttachmentOptionalParams;
use datadog_api_client::datadogV2::model::IncidentAttachmentType;
use datadog_api_client::datadogV2::model::PatchAttachmentRequest;
use datadog_api_client::datadogV2::model::PatchAttachmentRequestData;
use datadog_api_client::datadogV2::model::PatchAttachmentRequestDataAttributes;
use datadog_api_client::datadogV2::model::PatchAttachmentRequestDataAttributesAttachment;
use serde_json::Value;

#[tokio::main]
async fn main() {
    let body = PatchAttachmentRequest::new().data(
        PatchAttachmentRequestData::new(IncidentAttachmentType::INCIDENT_ATTACHMENTS).attributes(
            PatchAttachmentRequestDataAttributes::new().attachment(
                PatchAttachmentRequestDataAttributesAttachment::new()
                    .document_url(
                        "https://app.datadoghq.com/notebook/124/Postmortem-IR-124".to_string(),
                    )
                    .title("Postmortem-IR-124".to_string()),
            ),
        ),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdateIncidentAttachment", true);
    let api = IncidentsAPI::with_config(configuration);
    let resp = api
        .update_incident_attachment(
            "incident_id".to_string(),
            Value::from("00000000-0000-0000-0000-000000000002"),
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
