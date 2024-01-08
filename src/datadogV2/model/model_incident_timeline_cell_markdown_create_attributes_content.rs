// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The Markdown timeline cell contents.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentTimelineCellMarkdownCreateAttributesContent {
    /// The Markdown content of the cell.
    #[serde(rename = "content")]
    pub content: Option<String>,
}

impl IncidentTimelineCellMarkdownCreateAttributesContent {
    pub fn new() -> IncidentTimelineCellMarkdownCreateAttributesContent {
        IncidentTimelineCellMarkdownCreateAttributesContent { content: None }
    }
}
impl Default for IncidentTimelineCellMarkdownCreateAttributesContent {
    fn default() -> Self {
        Self::new()
    }
}
