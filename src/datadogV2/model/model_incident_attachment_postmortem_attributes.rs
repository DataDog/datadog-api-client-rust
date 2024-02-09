// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The attributes object for a postmortem attachment.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentAttachmentPostmortemAttributes {
    /// The postmortem attachment.
    #[serde(rename = "attachment")]
    pub attachment:
        crate::datadogV2::model::IncidentAttachmentsPostmortemAttributesAttachmentObject,
    /// The type of postmortem attachment attributes.
    #[serde(rename = "attachment_type")]
    pub attachment_type: crate::datadogV2::model::IncidentAttachmentPostmortemAttachmentType,
}

impl IncidentAttachmentPostmortemAttributes {
    pub fn new(
        attachment: crate::datadogV2::model::IncidentAttachmentsPostmortemAttributesAttachmentObject,
        attachment_type: crate::datadogV2::model::IncidentAttachmentPostmortemAttachmentType,
    ) -> IncidentAttachmentPostmortemAttributes {
        IncidentAttachmentPostmortemAttributes {
            attachment,
            attachment_type,
        }
    }
}
