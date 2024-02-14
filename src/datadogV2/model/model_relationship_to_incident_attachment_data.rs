// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The attachment relationship data.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RelationshipToIncidentAttachmentData {
    /// A unique identifier that represents the attachment.
    #[serde(rename = "id")]
    pub id: String,
    /// The incident attachment resource type.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::IncidentAttachmentType,
}

impl RelationshipToIncidentAttachmentData {
    pub fn new(
        id: String,
        type_: crate::datadogV2::model::IncidentAttachmentType,
    ) -> RelationshipToIncidentAttachmentData {
        RelationshipToIncidentAttachmentData { id, type_ }
    }
}
