// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The host map widget graphs any metric across your hosts using the same visualization available from the main Host Map page.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HostMapWidgetDefinition {
    /// List of custom links.
    #[serde(rename = "custom_links")]
    pub custom_links: Option<Vec<crate::datadogV1::model::WidgetCustomLink>>,
    /// List of tag prefixes to group by.
    #[serde(rename = "group")]
    pub group: Option<Vec<String>>,
    /// Whether to show the hosts that don’t fit in a group.
    #[serde(rename = "no_group_hosts")]
    pub no_group_hosts: Option<bool>,
    /// Whether to show the hosts with no metrics.
    #[serde(rename = "no_metric_hosts")]
    pub no_metric_hosts: Option<bool>,
    /// Which type of node to use in the map.
    #[serde(rename = "node_type")]
    pub node_type: Option<crate::datadogV1::model::WidgetNodeType>,
    /// Notes on the title.
    #[serde(rename = "notes")]
    pub notes: Option<String>,
    /// List of definitions.
    #[serde(rename = "requests")]
    pub requests: Box<crate::datadogV1::model::HostMapWidgetDefinitionRequests>,
    /// List of tags used to filter the map.
    #[serde(rename = "scope")]
    pub scope: Option<Vec<String>>,
    /// The style to apply to the widget.
    #[serde(rename = "style")]
    pub style: Option<Box<crate::datadogV1::model::HostMapWidgetDefinitionStyle>>,
    /// Title of the widget.
    #[serde(rename = "title")]
    pub title: Option<String>,
    /// How to align the text on the widget.
    #[serde(rename = "title_align")]
    pub title_align: Option<crate::datadogV1::model::WidgetTextAlign>,
    /// Size of the title.
    #[serde(rename = "title_size")]
    pub title_size: Option<String>,
    /// Type of the host map widget.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::HostMapWidgetDefinitionType,
}

impl HostMapWidgetDefinition {
    pub fn new(
        requests: Box<crate::datadogV1::model::HostMapWidgetDefinitionRequests>,
        type_: crate::datadogV1::model::HostMapWidgetDefinitionType,
    ) -> HostMapWidgetDefinition {
        HostMapWidgetDefinition {
            custom_links: None,
            group: None,
            no_group_hosts: None,
            no_metric_hosts: None,
            node_type: None,
            notes: None,
            requests,
            scope: None,
            style: None,
            title: None,
            title_align: None,
            title_size: None,
            type_,
        }
    }
}
