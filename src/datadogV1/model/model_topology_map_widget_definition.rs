// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// This widget displays a topology of nodes and edges for different data sources. It replaces the service map widget.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TopologyMapWidgetDefinition {
    /// List of custom links.
    #[serde(rename = "custom_links")]
    pub custom_links: Option<Vec<crate::datadogV1::model::WidgetCustomLink>>,
    /// One or more Topology requests.
    #[serde(rename = "requests")]
    pub requests: Vec<crate::datadogV1::model::TopologyRequest>,
    /// Title of your widget.
    #[serde(rename = "title")]
    pub title: Option<String>,
    /// How to align the text on the widget.
    #[serde(rename = "title_align")]
    pub title_align: Option<crate::datadogV1::model::WidgetTextAlign>,
    /// Size of the title.
    #[serde(rename = "title_size")]
    pub title_size: Option<String>,
    /// Type of the topology map widget.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::TopologyMapWidgetDefinitionType,
}

impl TopologyMapWidgetDefinition {
    pub fn new(
        requests: Vec<crate::datadogV1::model::TopologyRequest>,
        type_: crate::datadogV1::model::TopologyMapWidgetDefinitionType,
    ) -> TopologyMapWidgetDefinition {
        TopologyMapWidgetDefinition {
            custom_links: None,
            requests,
            title: None,
            title_align: None,
            title_size: None,
            type_,
        }
    }
}
