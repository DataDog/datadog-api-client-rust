// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentAttachmentPostmortemAttributes {
    /// The postmortem attachment.
    #[serde(rename = "attachment")]
    pub attachment: IncidentAttachmentsPostmortemAttributesAttachmentObject,
    /// The type of postmortem attachment attributes.
    #[serde(rename = "attachment_type", skip_serializing_if = "Option::is_none")]
    pub attachment_type: IncidentAttachmentPostmortemAttachmentType,
}

