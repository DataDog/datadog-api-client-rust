// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Synthetic global variable.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsGlobalVariable {
    /// Attributes of the global variable.
    #[serde(rename = "attributes")]
    pub attributes: Option<crate::datadogV1::model::SyntheticsGlobalVariableAttributes>,
    /// Description of the global variable.
    #[serde(rename = "description")]
    pub description: String,
    /// Unique identifier of the global variable.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Name of the global variable. Unique across Synthetic global variables.
    #[serde(rename = "name")]
    pub name: String,
    /// Parser options to use for retrieving a Synthetic global variable from a Synthetic test. Used in conjunction with `parse_test_public_id`.
    #[serde(rename = "parse_test_options")]
    pub parse_test_options:
        Option<crate::datadogV1::model::SyntheticsGlobalVariableParseTestOptions>,
    /// A Synthetic test ID to use as a test to generate the variable value.
    #[serde(rename = "parse_test_public_id")]
    pub parse_test_public_id: Option<String>,
    /// Tags of the global variable.
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
    /// Value of the global variable.
    #[serde(rename = "value")]
    pub value: crate::datadogV1::model::SyntheticsGlobalVariableValue,
}

impl SyntheticsGlobalVariable {
    pub fn new(
        description: String,
        name: String,
        tags: Vec<String>,
        value: crate::datadogV1::model::SyntheticsGlobalVariableValue,
    ) -> SyntheticsGlobalVariable {
        SyntheticsGlobalVariable {
            attributes: None,
            description,
            id: None,
            name,
            parse_test_options: None,
            parse_test_public_id: None,
            tags,
            value,
        }
    }

    pub fn attributes(
        mut self,
        value: crate::datadogV1::model::SyntheticsGlobalVariableAttributes,
    ) -> Self {
        self.attributes = Some(value);
        self
    }

    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }

    pub fn parse_test_options(
        mut self,
        value: crate::datadogV1::model::SyntheticsGlobalVariableParseTestOptions,
    ) -> Self {
        self.parse_test_options = Some(value);
        self
    }

    pub fn parse_test_public_id(mut self, value: String) -> Self {
        self.parse_test_public_id = Some(value);
        self
    }
}
