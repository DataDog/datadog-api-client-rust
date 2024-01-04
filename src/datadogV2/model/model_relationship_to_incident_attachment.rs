// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A relationship reference for attachments.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RelationshipToIncidentAttachment {
    /// An array of incident attachments.
    #[serde(rename = "data")]
    pub data: Vec<crate::datadogV2::model::RelationshipToIncidentAttachmentData>,
}

impl RelationshipToIncidentAttachment {
    pub fn new(
        data: Vec<crate::datadogV2::model::RelationshipToIncidentAttachmentData>,
    ) -> RelationshipToIncidentAttachment {
        RelationshipToIncidentAttachment { data }
    }
}
