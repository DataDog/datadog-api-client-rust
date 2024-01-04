// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Powerpack group widget object.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PowerpackGroupWidgetDefinition {
    /// Layout type of widgets.
    #[serde(rename = "layout_type")]
    pub layout_type: String,
    /// Boolean indicating whether powerpack group title should be visible or not.
    #[serde(rename = "show_title")]
    pub show_title: Option<bool>,
    /// Name for the group widget.
    #[serde(rename = "title")]
    pub title: Option<String>,
    /// Type of widget, must be group.
    #[serde(rename = "type")]
    pub type_: String,
    /// Widgets inside the powerpack.
    #[serde(rename = "widgets")]
    pub widgets: Vec<crate::datadogV2::model::PowerpackInnerWidgets>,
}

impl PowerpackGroupWidgetDefinition {
    pub fn new(
        layout_type: String,
        type_: String,
        widgets: Vec<crate::datadogV2::model::PowerpackInnerWidgets>,
    ) -> PowerpackGroupWidgetDefinition {
        PowerpackGroupWidgetDefinition {
            layout_type,
            show_title: None,
            title: None,
            type_,
            widgets,
        }
    }
}
