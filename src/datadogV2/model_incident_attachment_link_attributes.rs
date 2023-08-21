// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentAttachmentLinkAttributes {
    /// The link attachment.
    #[serde(rename = "attachment")]
    pub attachment: IncidentAttachmentLinkAttributesAttachmentObject,
    /// The type of link attachment attributes.
    #[serde(rename = "attachment_type", skip_serializing_if = "Option::is_none")]
    pub attachment_type: IncidentAttachmentLinkAttachmentType,
}

