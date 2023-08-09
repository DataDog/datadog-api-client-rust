/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// IncidentAttachmentLinkAttributesAttachmentObject : The link attachment.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct IncidentAttachmentLinkAttributesAttachmentObject {
    /// The URL of this link attachment.
    #[serde(rename = "documentUrl")]
    pub document_url: String,
    /// The title of this link attachment.
    #[serde(rename = "title")]
    pub title: String,
}

impl IncidentAttachmentLinkAttributesAttachmentObject {
    /// The link attachment.
    pub fn new(document_url: String, title: String) -> IncidentAttachmentLinkAttributesAttachmentObject {
        IncidentAttachmentLinkAttributesAttachmentObject {
            document_url,
            title,
        }
    }
}


