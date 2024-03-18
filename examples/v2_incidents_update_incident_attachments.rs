// Create, update, and delete incident attachments returns "OK" response
use chrono::prelude::{DateTime, Utc};
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_incidents::*;
use datadog_api_client::datadogV2::model::*;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let body = IncidentAttachmentUpdateRequest::new(vec![
        IncidentAttachmentUpdateData::new(IncidentAttachmentType::INCIDENT_ATTACHMENTS)
            .attributes(
                IncidentAttachmentUpdateAttributes::IncidentAttachmentPostmortemAttributes(
                    Box::new(IncidentAttachmentPostmortemAttributes::new(
                        IncidentAttachmentsPostmortemAttributesAttachmentObject::new(
                            "https://app.datadoghq.com/notebook/123".to_string(),
                            "Postmortem IR-123".to_string(),
                        ),
                        IncidentAttachmentPostmortemAttachmentType::POSTMORTEM,
                    )),
                ),
            )
            .id("00000000-abcd-0002-0000-000000000000".to_string()),
        IncidentAttachmentUpdateData::new(IncidentAttachmentType::INCIDENT_ATTACHMENTS).attributes(
            IncidentAttachmentUpdateAttributes::IncidentAttachmentLinkAttributes(Box::new(
                IncidentAttachmentLinkAttributes::new(
                    IncidentAttachmentLinkAttributesAttachmentObject::new(
                        "https://www.example.com/webstore-failure-runbook".to_string(),
                        "Runbook for webstore service failures".to_string(),
                    ),
                    IncidentAttachmentLinkAttachmentType::LINK,
                ),
            )),
        ),
        IncidentAttachmentUpdateData::new(IncidentAttachmentType::INCIDENT_ATTACHMENTS)
            .id("00000000-abcd-0003-0000-000000000000".to_string()),
    ]);
    let mut configuration = Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdateIncidentAttachments", true);
    let api = IncidentsAPI::with_config(configuration);
    let resp = api
        .update_incident_attachments(
            "incident_id".to_string(),
            body,
            UpdateIncidentAttachmentsOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
