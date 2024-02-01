// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The split graph widget allows you to create repeating units of a graph - one for each value in a group (for example: one per service)
///
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SplitGraphWidgetDefinition {
    /// Normalize y axes across graphs
    #[serde(rename = "has_uniform_y_axes")]
    pub has_uniform_y_axes: Option<bool>,
    /// Size of the individual graphs in the split.
    #[serde(rename = "size")]
    pub size: crate::datadogV1::model::SplitGraphVizSize,
    /// The original widget we are splitting on.
    #[serde(rename = "source_widget_definition")]
    pub source_widget_definition: crate::datadogV1::model::SplitGraphSourceWidgetDefinition,
    /// Encapsulates all user choices about how to split a graph.
    #[serde(rename = "split_config")]
    pub split_config: crate::datadogV1::model::SplitConfig,
    /// Time setting for the widget.
    #[serde(rename = "time")]
    pub time: Option<crate::datadogV1::model::WidgetTime>,
    /// Title of your widget.
    #[serde(rename = "title")]
    pub title: Option<String>,
    /// Type of the split graph widget
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::SplitGraphWidgetDefinitionType,
}

impl SplitGraphWidgetDefinition {
    pub fn new(
        size: crate::datadogV1::model::SplitGraphVizSize,
        source_widget_definition: crate::datadogV1::model::SplitGraphSourceWidgetDefinition,
        split_config: crate::datadogV1::model::SplitConfig,
        type_: crate::datadogV1::model::SplitGraphWidgetDefinitionType,
    ) -> SplitGraphWidgetDefinition {
        SplitGraphWidgetDefinition {
            has_uniform_y_axes: None,
            size,
            source_widget_definition,
            split_config,
            time: None,
            title: None,
            type_,
        }
    }

    pub fn has_uniform_y_axes(&mut self, value: bool) -> &mut Self {
        self.has_uniform_y_axes = Some(value);
        self
    }

    pub fn time(&mut self, value: crate::datadogV1::model::WidgetTime) -> &mut Self {
        self.time = Some(value);
        self
    }

    pub fn title(&mut self, value: String) -> &mut Self {
        self.title = Some(value);
        self
    }
}
