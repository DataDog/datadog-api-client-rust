// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A single incident attachment.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentAttachmentUpdateData {
    /// Incident attachment attributes.
    #[serde(rename = "attributes")]
    pub attributes: Option<Box<crate::datadogV2::model::IncidentAttachmentUpdateAttributes>>,
    /// A unique identifier that represents the incident attachment.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// The incident attachment resource type.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::IncidentAttachmentType,
}

impl IncidentAttachmentUpdateData {
    pub fn new(
        type_: crate::datadogV2::model::IncidentAttachmentType,
    ) -> IncidentAttachmentUpdateData {
        IncidentAttachmentUpdateData {
            attributes: None,
            id: None,
            type_,
        }
    }
}
