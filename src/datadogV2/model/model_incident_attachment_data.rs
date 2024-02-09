// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A single incident attachment.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentAttachmentData {
    /// The attributes object for an attachment.
    #[serde(rename = "attributes")]
    pub attributes: crate::datadogV2::model::IncidentAttachmentAttributes,
    /// A unique identifier that represents the incident attachment.
    #[serde(rename = "id")]
    pub id: String,
    /// The incident attachment's relationships.
    #[serde(rename = "relationships")]
    pub relationships: crate::datadogV2::model::IncidentAttachmentRelationships,
    /// The incident attachment resource type.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::IncidentAttachmentType,
}

impl IncidentAttachmentData {
    pub fn new(
        attributes: crate::datadogV2::model::IncidentAttachmentAttributes,
        id: String,
        relationships: crate::datadogV2::model::IncidentAttachmentRelationships,
        type_: crate::datadogV2::model::IncidentAttachmentType,
    ) -> IncidentAttachmentData {
        IncidentAttachmentData {
            attributes,
            id,
            relationships,
            type_,
        }
    }
}
