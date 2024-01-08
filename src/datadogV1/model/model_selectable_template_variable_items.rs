// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object containing the template variable's name, associated tag/attribute, default value and selectable values.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SelectableTemplateVariableItems {
    /// The default value of the template variable.
    #[serde(rename = "default_value")]
    pub default_value: Option<String>,
    /// Name of the template variable.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// The tag/attribute key associated with the template variable.
    #[serde(rename = "prefix")]
    pub prefix: Option<String>,
    /// List of visible tag values on the shared dashboard.
    #[serde(
        rename = "visible_tags",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub visible_tags: Option<Option<Vec<String>>>,
}

impl SelectableTemplateVariableItems {
    pub fn new() -> SelectableTemplateVariableItems {
        SelectableTemplateVariableItems {
            default_value: None,
            name: None,
            prefix: None,
            visible_tags: None,
        }
    }
}
impl Default for SelectableTemplateVariableItems {
    fn default() -> Self {
        Self::new()
    }
}
