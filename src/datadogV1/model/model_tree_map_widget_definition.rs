// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The treemap visualization enables you to display hierarchical and nested data. It is well suited for queries that describe part-whole relationships, such as resource usage by availability zone, data center, or team.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TreeMapWidgetDefinition {
    /// (deprecated) The attribute formerly used to determine color in the widget.
    #[deprecated]
    #[serde(rename = "color_by")]
    pub color_by: Option<crate::datadogV1::model::TreeMapColorBy>,
    /// List of custom links.
    #[serde(rename = "custom_links")]
    pub custom_links: Option<Vec<crate::datadogV1::model::WidgetCustomLink>>,
    /// (deprecated) The attribute formerly used to group elements in the widget.
    #[deprecated]
    #[serde(rename = "group_by")]
    pub group_by: Option<crate::datadogV1::model::TreeMapGroupBy>,
    /// List of treemap widget requests.
    #[serde(rename = "requests")]
    pub requests: Vec<crate::datadogV1::model::TreeMapWidgetRequest>,
    /// (deprecated) The attribute formerly used to determine size in the widget.
    #[deprecated]
    #[serde(rename = "size_by")]
    pub size_by: Option<crate::datadogV1::model::TreeMapSizeBy>,
    /// Time setting for the widget.
    #[serde(rename = "time")]
    pub time: Option<crate::datadogV1::model::WidgetTime>,
    /// Title of your widget.
    #[serde(rename = "title")]
    pub title: Option<String>,
    /// Type of the treemap widget.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::TreeMapWidgetDefinitionType,
}

impl TreeMapWidgetDefinition {
    pub fn new(
        requests: Vec<crate::datadogV1::model::TreeMapWidgetRequest>,
        type_: crate::datadogV1::model::TreeMapWidgetDefinitionType,
    ) -> TreeMapWidgetDefinition {
        #[allow(deprecated)]
        TreeMapWidgetDefinition {
            color_by: None,
            custom_links: None,
            group_by: None,
            requests,
            size_by: None,
            time: None,
            title: None,
            type_,
        }
    }

    #[allow(deprecated)]
    pub fn color_by(&mut self, value: crate::datadogV1::model::TreeMapColorBy) -> &mut Self {
        self.color_by = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn custom_links(
        &mut self,
        value: Vec<crate::datadogV1::model::WidgetCustomLink>,
    ) -> &mut Self {
        self.custom_links = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn group_by(&mut self, value: crate::datadogV1::model::TreeMapGroupBy) -> &mut Self {
        self.group_by = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn size_by(&mut self, value: crate::datadogV1::model::TreeMapSizeBy) -> &mut Self {
        self.size_by = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn time(&mut self, value: crate::datadogV1::model::WidgetTime) -> &mut Self {
        self.time = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn title(&mut self, value: String) -> &mut Self {
        self.title = Some(value);
        self
    }
}
