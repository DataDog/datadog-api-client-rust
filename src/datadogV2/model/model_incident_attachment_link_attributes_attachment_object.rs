// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The link attachment.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentAttachmentLinkAttributesAttachmentObject {
    /// The URL of this link attachment.
    #[serde(rename = "documentUrl")]
    pub document_url: String,
    /// The title of this link attachment.
    #[serde(rename = "title")]
    pub title: String,
}

impl IncidentAttachmentLinkAttributesAttachmentObject {
    pub fn new(
        document_url: String,
        title: String,
    ) -> IncidentAttachmentLinkAttributesAttachmentObject {
        IncidentAttachmentLinkAttributesAttachmentObject {
            document_url,
            title,
        }
    }
}
