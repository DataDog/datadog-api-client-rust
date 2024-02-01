// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A message containing the response to a scalar query.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScalarResponse {
    /// The object describing a scalar response.
    #[serde(rename = "attributes")]
    pub attributes: Option<crate::datadogV2::model::ScalarFormulaResponseAtrributes>,
    /// The type of the resource. The value should always be scalar_response.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::ScalarFormulaResponseType>,
}

impl ScalarResponse {
    pub fn new() -> ScalarResponse {
        ScalarResponse {
            attributes: None,
            type_: None,
        }
    }

    pub fn attributes(
        &mut self,
        value: crate::datadogV2::model::ScalarFormulaResponseAtrributes,
    ) -> &mut Self {
        self.attributes = Some(value);
        self
    }

    pub fn type_(
        &mut self,
        value: crate::datadogV2::model::ScalarFormulaResponseType,
    ) -> &mut Self {
        self.type_ = Some(value);
        self
    }
}

impl Default for ScalarResponse {
    fn default() -> Self {
        Self::new()
    }
}
