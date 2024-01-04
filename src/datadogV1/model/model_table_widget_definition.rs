// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The table visualization is available on timeboards and screenboards. It displays columns of metrics grouped by tag key.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TableWidgetDefinition {
    /// List of custom links.
    #[serde(rename = "custom_links")]
    pub custom_links: Option<Vec<crate::datadogV1::model::WidgetCustomLink>>,
    /// Controls the display of the search bar.
    #[serde(rename = "has_search_bar")]
    pub has_search_bar: Option<crate::datadogV1::model::TableWidgetHasSearchBar>,
    /// Widget definition.
    #[serde(rename = "requests")]
    pub requests: Vec<crate::datadogV1::model::TableWidgetRequest>,
    /// Time setting for the widget.
    #[serde(rename = "time")]
    pub time: Option<Box<crate::datadogV1::model::WidgetTime>>,
    /// Title of your widget.
    #[serde(rename = "title")]
    pub title: Option<String>,
    /// How to align the text on the widget.
    #[serde(rename = "title_align")]
    pub title_align: Option<crate::datadogV1::model::WidgetTextAlign>,
    /// Size of the title.
    #[serde(rename = "title_size")]
    pub title_size: Option<String>,
    /// Type of the table widget.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::TableWidgetDefinitionType,
}

impl TableWidgetDefinition {
    pub fn new(
        requests: Vec<crate::datadogV1::model::TableWidgetRequest>,
        type_: crate::datadogV1::model::TableWidgetDefinitionType,
    ) -> TableWidgetDefinition {
        TableWidgetDefinition {
            custom_links: None,
            has_search_bar: None,
            requests,
            time: None,
            title: None,
            title_align: None,
            title_size: None,
            type_,
        }
    }
}
