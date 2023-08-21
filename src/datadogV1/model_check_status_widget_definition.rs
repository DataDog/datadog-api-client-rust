// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckStatusWidgetDefinition {
    /// Name of the check to use in the widget.
    #[serde(rename = "check", skip_serializing_if = "Option::is_none")]
    pub check: String,
    /// Group reporting a single check.
    #[serde(rename = "group", skip_serializing_if = "Option::is_none")]
    pub group: String,
    /// List of tag prefixes to group by in the case of a cluster check.
    #[serde(rename = "group_by", skip_serializing_if = "Option::is_none")]
    pub group_by: Vec<String>,
    /// The kind of grouping to use.
    #[serde(rename = "grouping", skip_serializing_if = "Option::is_none")]
    pub grouping: WidgetGrouping,
    /// List of tags used to filter the groups reporting a cluster check.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Vec<String>,
    /// Time setting for the widget.
    #[serde(rename = "time", skip_serializing_if = "Option::is_none")]
    pub time: WidgetTime,
    /// Title of the widget.
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: String,
    /// How to align the text on the widget.
    #[serde(rename = "title_align", skip_serializing_if = "Option::is_none")]
    pub title_align: WidgetTextAlign,
    /// Size of the title.
    #[serde(rename = "title_size", skip_serializing_if = "Option::is_none")]
    pub title_size: String,
    /// Type of the check status widget.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: CheckStatusWidgetDefinitionType,
}

