/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// IncidentAttachmentAttachmentType : The type of the incident attachment attributes.

/// The type of the incident attachment attributes.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum IncidentAttachmentAttachmentType {
    #[serde(rename = "link")]
    LINK,
    #[serde(rename = "postmortem")]
    POSTMORTEM,

}

impl ToString for IncidentAttachmentAttachmentType {
    fn to_string(&self) -> String {
        match self {
            Self::LINK => String::from("link"),
            Self::POSTMORTEM => String::from("postmortem"),
        }
    }
}

impl Default for IncidentAttachmentAttachmentType {
    fn default() -> IncidentAttachmentAttachmentType {
        Self::LINK
    }
}




