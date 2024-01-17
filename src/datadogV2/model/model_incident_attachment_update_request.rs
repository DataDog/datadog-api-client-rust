// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The update request for an incident's attachments.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentAttachmentUpdateRequest {
    /// An array of incident attachments. An attachment object without an "id" key indicates that you want to
    /// create that attachment. An attachment object without an "attributes" key indicates that you want to
    /// delete that attachment. An attachment object with both the "id" key and a populated "attributes" object
    /// indicates that you want to update that attachment.
    #[serde(rename = "data")]
    pub data: Vec<crate::datadogV2::model::IncidentAttachmentUpdateData>,
}

impl IncidentAttachmentUpdateRequest {
    pub fn new(
        data: Vec<crate::datadogV2::model::IncidentAttachmentUpdateData>,
    ) -> IncidentAttachmentUpdateRequest {
        IncidentAttachmentUpdateRequest { data }
    }
}
