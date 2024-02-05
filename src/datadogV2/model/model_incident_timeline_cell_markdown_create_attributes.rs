// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Timeline cell data for Markdown timeline cells for a create request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentTimelineCellMarkdownCreateAttributes {
    /// Type of the Markdown timeline cell.
    #[serde(rename = "cell_type")]
    pub cell_type: crate::datadogV2::model::IncidentTimelineCellMarkdownContentType,
    /// The Markdown timeline cell contents.
    #[serde(rename = "content")]
    pub content: crate::datadogV2::model::IncidentTimelineCellMarkdownCreateAttributesContent,
    /// A flag indicating whether the timeline cell is important and should be highlighted.
    #[serde(rename = "important")]
    pub important: Option<bool>,
}

impl IncidentTimelineCellMarkdownCreateAttributes {
    pub fn new(
        cell_type: crate::datadogV2::model::IncidentTimelineCellMarkdownContentType,
        content: crate::datadogV2::model::IncidentTimelineCellMarkdownCreateAttributesContent,
    ) -> IncidentTimelineCellMarkdownCreateAttributes {
        IncidentTimelineCellMarkdownCreateAttributes {
            cell_type,
            content,
            important: None,
        }
    }

    pub fn important(&mut self, value: bool) -> &mut Self {
        self.important = Some(value);
        self
    }
}
