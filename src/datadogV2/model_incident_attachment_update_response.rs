// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentAttachmentUpdateResponse {
    /// An array of incident attachments. Only the attachments that were created or updated by the request are
returned.
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Vec<IncidentAttachmentData>,
    /// Included related resources that the user requested.
    #[serde(rename = "included", skip_serializing_if = "Option::is_none")]
    pub included: Vec<IncidentAttachmentsResponseIncludedItem>,
}

