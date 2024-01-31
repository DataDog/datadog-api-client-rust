// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Check status shows the current status or number of results for any check performed.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckStatusWidgetDefinition {
    /// Name of the check to use in the widget.
    #[serde(rename = "check")]
    pub check: String,
    /// Group reporting a single check.
    #[serde(rename = "group")]
    pub group: Option<String>,
    /// List of tag prefixes to group by in the case of a cluster check.
    #[serde(rename = "group_by")]
    pub group_by: Option<Vec<String>>,
    /// The kind of grouping to use.
    #[serde(rename = "grouping")]
    pub grouping: crate::datadogV1::model::WidgetGrouping,
    /// List of tags used to filter the groups reporting a cluster check.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// Time setting for the widget.
    #[serde(rename = "time")]
    pub time: Option<crate::datadogV1::model::WidgetTime>,
    /// Title of the widget.
    #[serde(rename = "title")]
    pub title: Option<String>,
    /// How to align the text on the widget.
    #[serde(rename = "title_align")]
    pub title_align: Option<crate::datadogV1::model::WidgetTextAlign>,
    /// Size of the title.
    #[serde(rename = "title_size")]
    pub title_size: Option<String>,
    /// Type of the check status widget.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::CheckStatusWidgetDefinitionType,
}

impl CheckStatusWidgetDefinition {
    pub fn new(
        check: String,
        grouping: crate::datadogV1::model::WidgetGrouping,
        type_: crate::datadogV1::model::CheckStatusWidgetDefinitionType,
    ) -> CheckStatusWidgetDefinition {
        CheckStatusWidgetDefinition {
            check,
            group: None,
            group_by: None,
            grouping,
            tags: None,
            time: None,
            title: None,
            title_align: None,
            title_size: None,
            type_,
        }
    }

    pub fn with_group(&mut self, value: String) -> &mut Self {
        self.group = Some(value);
        self
    }

    pub fn with_group_by(&mut self, value: Vec<String>) -> &mut Self {
        self.group_by = Some(value);
        self
    }

    pub fn with_tags(&mut self, value: Vec<String>) -> &mut Self {
        self.tags = Some(value);
        self
    }

    pub fn with_time(&mut self, value: crate::datadogV1::model::WidgetTime) -> &mut Self {
        self.time = Some(value);
        self
    }

    pub fn with_title(&mut self, value: String) -> &mut Self {
        self.title = Some(value);
        self
    }

    pub fn with_title_align(
        &mut self,
        value: crate::datadogV1::model::WidgetTextAlign,
    ) -> &mut Self {
        self.title_align = Some(value);
        self
    }

    pub fn with_title_size(&mut self, value: String) -> &mut Self {
        self.title_size = Some(value);
        self
    }
}
