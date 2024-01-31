// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The response object containing an incident's attachments.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentAttachmentsResponse {
    /// An array of incident attachments.
    #[serde(rename = "data")]
    pub data: Vec<crate::datadogV2::model::IncidentAttachmentData>,
    /// Included related resources that the user requested.
    #[serde(rename = "included")]
    pub included: Option<Vec<crate::datadogV2::model::IncidentAttachmentsResponseIncludedItem>>,
}

impl IncidentAttachmentsResponse {
    pub fn new(
        data: Vec<crate::datadogV2::model::IncidentAttachmentData>,
    ) -> IncidentAttachmentsResponse {
        IncidentAttachmentsResponse {
            data,
            included: None,
        }
    }

    pub fn with_included(
        &mut self,
        value: Vec<crate::datadogV2::model::IncidentAttachmentsResponseIncludedItem>,
    ) -> &mut Self {
        self.included = Some(value);
        self
    }
}
