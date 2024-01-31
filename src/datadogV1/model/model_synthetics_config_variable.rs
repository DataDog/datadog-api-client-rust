// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object defining a variable that can be used in your test configuration.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsConfigVariable {
    /// Example for the variable.
    #[serde(rename = "example")]
    pub example: Option<String>,
    /// ID of the variable for global variables.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Name of the variable.
    #[serde(rename = "name")]
    pub name: String,
    /// Pattern of the variable.
    #[serde(rename = "pattern")]
    pub pattern: Option<String>,
    /// Whether the value of this variable will be obfuscated in test results. Only for config variables of type `text`.
    #[serde(rename = "secure")]
    pub secure: Option<bool>,
    /// Type of the configuration variable.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::SyntheticsConfigVariableType,
}

impl SyntheticsConfigVariable {
    pub fn new(
        name: String,
        type_: crate::datadogV1::model::SyntheticsConfigVariableType,
    ) -> SyntheticsConfigVariable {
        SyntheticsConfigVariable {
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
