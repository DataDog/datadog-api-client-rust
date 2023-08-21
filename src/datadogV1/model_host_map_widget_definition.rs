// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HostMapWidgetDefinition {
    /// List of custom links.
    #[serde(rename = "custom_links", skip_serializing_if = "Option::is_none")]
    pub custom_links: Vec<WidgetCustomLink>,
    /// List of tag prefixes to group by.
    #[serde(rename = "group", skip_serializing_if = "Option::is_none")]
    pub group: Vec<String>,
    /// Whether to show the hosts that don’t fit in a group.
    #[serde(rename = "no_group_hosts", skip_serializing_if = "Option::is_none")]
    pub no_group_hosts: bool,
    /// Whether to show the hosts with no metrics.
    #[serde(rename = "no_metric_hosts", skip_serializing_if = "Option::is_none")]
    pub no_metric_hosts: bool,
    /// Which type of node to use in the map.
    #[serde(rename = "node_type", skip_serializing_if = "Option::is_none")]
    pub node_type: WidgetNodeType,
    /// Notes on the title.
    #[serde(rename = "notes", skip_serializing_if = "Option::is_none")]
    pub notes: String,
    /// List of definitions.
    #[serde(rename = "requests", skip_serializing_if = "Option::is_none")]
    pub requests: HostMapWidgetDefinitionRequests,
    /// List of tags used to filter the map.
    #[serde(rename = "scope", skip_serializing_if = "Option::is_none")]
    pub scope: Vec<String>,
    /// The style to apply to the widget.
    #[serde(rename = "style", skip_serializing_if = "Option::is_none")]
    pub style: HostMapWidgetDefinitionStyle,
    /// Title of the widget.
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: String,
    /// How to align the text on the widget.
    #[serde(rename = "title_align", skip_serializing_if = "Option::is_none")]
    pub title_align: WidgetTextAlign,
    /// Size of the title.
    #[serde(rename = "title_size", skip_serializing_if = "Option::is_none")]
    pub title_size: String,
    /// Type of the host map widget.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: HostMapWidgetDefinitionType,
}

