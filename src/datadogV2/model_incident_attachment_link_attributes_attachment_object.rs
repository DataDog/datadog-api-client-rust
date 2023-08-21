// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentAttachmentLinkAttributesAttachmentObject {
    /// The URL of this link attachment.
    #[serde(rename = "documentUrl", skip_serializing_if = "Option::is_none")]
    pub document_url: String,
    /// The title of this link attachment.
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: String,
}

