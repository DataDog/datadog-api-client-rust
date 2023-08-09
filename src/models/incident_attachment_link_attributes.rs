/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// IncidentAttachmentLinkAttributes : The attributes object for a link attachment.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct IncidentAttachmentLinkAttributes {
    #[serde(rename = "attachment")]
    pub attachment: Box<crate::models::IncidentAttachmentLinkAttributesAttachmentObject>,
    #[serde(rename = "attachment_type")]
    pub attachment_type: crate::models::IncidentAttachmentLinkAttachmentType,
}

impl IncidentAttachmentLinkAttributes {
    /// The attributes object for a link attachment.
    pub fn new(attachment: crate::models::IncidentAttachmentLinkAttributesAttachmentObject, attachment_type: crate::models::IncidentAttachmentLinkAttachmentType) -> IncidentAttachmentLinkAttributes {
        IncidentAttachmentLinkAttributes {
            attachment: Box::new(attachment),
            attachment_type,
        }
    }
}


