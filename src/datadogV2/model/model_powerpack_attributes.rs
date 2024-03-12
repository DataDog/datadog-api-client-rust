// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Powerpack attribute object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PowerpackAttributes {
    /// Description of this powerpack.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Powerpack group widget definition object.
    #[serde(rename = "group_widget")]
    pub group_widget: crate::datadogV2::model::PowerpackGroupWidget,
    /// Name of the powerpack.
    #[serde(rename = "name")]
    pub name: String,
    /// List of tags to identify this powerpack.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// List of template variables for this powerpack.
    #[serde(rename = "template_variables")]
    pub template_variables: Option<Vec<crate::datadogV2::model::PowerpackTemplateVariable>>,
}

impl PowerpackAttributes {
    pub fn new(
        group_widget: crate::datadogV2::model::PowerpackGroupWidget,
        name: String,
    ) -> PowerpackAttributes {
        PowerpackAttributes {
            description: None,
            group_widget,
            name,
            tags: None,
            template_variables: None,
        }
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn template_variables(
        mut self,
        value: Vec<crate::datadogV2::model::PowerpackTemplateVariable>,
    ) -> Self {
        self.template_variables = Some(value);
        self
    }
}
