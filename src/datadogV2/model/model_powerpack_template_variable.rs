// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Powerpack template variables.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PowerpackTemplateVariable {
    /// One or many template variable default values within the saved view, which are unioned together using `OR` if more than one is specified.
    #[serde(rename = "defaults")]
    pub defaults: Option<Vec<String>>,
    /// The name of the variable.
    #[serde(rename = "name")]
    pub name: String,
}

impl PowerpackTemplateVariable {
    pub fn new(name: String) -> PowerpackTemplateVariable {
        PowerpackTemplateVariable {
            defaults: None,
            name,
        }
    }

    pub fn defaults(&mut self, value: Vec<String>) -> &mut Self {
        self.defaults = Some(value);
        self
    }
}
