// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object defining a variable that can be used in your browser test.
/// See the [Recording Steps documentation](<https://docs.datadoghq.com/synthetics/browser_tests/actions/?tab=testanelementontheactivepage#variables>).
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsBrowserVariable {
    /// Example for the variable.
    #[serde(rename = "example")]
    pub example: Option<String>,
    /// ID for the variable. Global variables require an ID.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Name of the variable.
    #[serde(rename = "name")]
    pub name: String,
    /// Pattern of the variable.
    #[serde(rename = "pattern")]
    pub pattern: Option<String>,
    /// Determines whether or not the browser test variable is obfuscated. Can only be used with browser variables of type `text`.
    #[serde(rename = "secure")]
    pub secure: Option<bool>,
    /// Type of browser test variable.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::SyntheticsBrowserVariableType,
}

impl SyntheticsBrowserVariable {
    pub fn new(
        name: String,
        type_: crate::datadogV1::model::SyntheticsBrowserVariableType,
    ) -> SyntheticsBrowserVariable {
        SyntheticsBrowserVariable {
            example: None,
            id: None,
            name,
            pattern: None,
            secure: None,
            type_,
        }
    }

    pub fn with_example(&mut self, value: String) -> &mut Self {
        self.example = Some(value);
        self
    }

    pub fn with_id(&mut self, value: String) -> &mut Self {
        self.id = Some(value);
        self
    }

    pub fn with_pattern(&mut self, value: String) -> &mut Self {
        self.pattern = Some(value);
        self
    }

    pub fn with_secure(&mut self, value: bool) -> &mut Self {
        self.secure = Some(value);
        self
    }
}
