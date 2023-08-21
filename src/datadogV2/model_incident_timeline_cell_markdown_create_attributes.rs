// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentTimelineCellMarkdownCreateAttributes {
    /// Type of the Markdown timeline cell.
    #[serde(rename = "cell_type", skip_serializing_if = "Option::is_none")]
    pub cell_type: IncidentTimelineCellMarkdownContentType,
    /// The Markdown timeline cell contents.
    #[serde(rename = "content", skip_serializing_if = "Option::is_none")]
    pub content: IncidentTimelineCellMarkdownCreateAttributesContent,
    /// A flag indicating whether the timeline cell is important and should be highlighted.
    #[serde(rename = "important", skip_serializing_if = "Option::is_none")]
    pub important: bool,
}

