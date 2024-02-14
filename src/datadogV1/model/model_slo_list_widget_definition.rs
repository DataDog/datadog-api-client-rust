// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Use the SLO List widget to track your SLOs (Service Level Objectives) on dashboards.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SLOListWidgetDefinition {
    /// Array of one request object to display in the widget.
    #[serde(rename = "requests")]
    pub requests: Vec<crate::datadogV1::model::SLOListWidgetRequest>,
    /// Title of the widget.
    #[serde(rename = "title")]
    pub title: Option<String>,
    /// How to align the text on the widget.
    #[serde(rename = "title_align")]
    pub title_align: Option<crate::datadogV1::model::WidgetTextAlign>,
    /// Size of the title.
    #[serde(rename = "title_size")]
    pub title_size: Option<String>,
    /// Type of the SLO List widget.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::SLOListWidgetDefinitionType,
}

impl SLOListWidgetDefinition {
    pub fn new(
        requests: Vec<crate::datadogV1::model::SLOListWidgetRequest>,
        type_: crate::datadogV1::model::SLOListWidgetDefinitionType,
    ) -> SLOListWidgetDefinition {
        SLOListWidgetDefinition {
            requests,
            title: None,
            title_align: None,
            title_size: None,
            type_,
        }
    }

    pub fn title(&mut self, value: String) -> &mut Self {
        self.title = Some(value);
        self
    }

    pub fn title_align(&mut self, value: crate::datadogV1::model::WidgetTextAlign) -> &mut Self {
        self.title_align = Some(value);
        self
    }

    pub fn title_size(&mut self, value: String) -> &mut Self {
        self.title_size = Some(value);
        self
    }
}
