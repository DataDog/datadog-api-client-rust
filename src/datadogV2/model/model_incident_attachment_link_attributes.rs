// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The attributes object for a link attachment.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentAttachmentLinkAttributes {
    /// The link attachment.
    #[serde(rename = "attachment")]
    pub attachment: crate::datadogV2::model::IncidentAttachmentLinkAttributesAttachmentObject,
    /// The type of link attachment attributes.
    #[serde(rename = "attachment_type")]
    pub attachment_type: crate::datadogV2::model::IncidentAttachmentLinkAttachmentType,
    /// Timestamp when the incident attachment link was last modified.
    #[serde(rename = "modified")]
    pub modified: Option<String>,
}

impl IncidentAttachmentLinkAttributes {
    pub fn new(
        attachment: crate::datadogV2::model::IncidentAttachmentLinkAttributesAttachmentObject,
        attachment_type: crate::datadogV2::model::IncidentAttachmentLinkAttachmentType,
    ) -> IncidentAttachmentLinkAttributes {
        IncidentAttachmentLinkAttributes {
            attachment,
            attachment_type,
            modified: None,
        }
    }

    pub fn modified(mut self, value: String) -> Self {
        self.modified = Some(value);
        self
    }
}
