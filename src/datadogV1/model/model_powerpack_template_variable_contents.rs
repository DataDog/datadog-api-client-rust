// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Powerpack template variable contents.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PowerpackTemplateVariableContents {
    /// The name of the variable.
    #[serde(rename = "name")]
    pub name: String,
    /// The tag prefix associated with the variable.
    #[serde(rename = "prefix")]
    pub prefix: Option<String>,
    /// One or many template variable values within the saved view, which will be unioned together using `OR` if more than one is specified.
    #[serde(rename = "values")]
    pub values: Vec<String>,
}

impl PowerpackTemplateVariableContents {
    pub fn new(name: String, values: Vec<String>) -> PowerpackTemplateVariableContents {
        PowerpackTemplateVariableContents {
            name,
            prefix: None,
            values,
        }
    }
}
