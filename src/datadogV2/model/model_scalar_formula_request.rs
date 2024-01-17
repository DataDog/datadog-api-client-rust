// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A single scalar query to be executed.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScalarFormulaRequest {
    /// The object describing a scalar formula request.
    #[serde(rename = "attributes")]
    pub attributes: Box<crate::datadogV2::model::ScalarFormulaRequestAttributes>,
    /// The type of the resource. The value should always be scalar_request.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::ScalarFormulaRequestType,
}

impl ScalarFormulaRequest {
    pub fn new(
        attributes: Box<crate::datadogV2::model::ScalarFormulaRequestAttributes>,
        type_: crate::datadogV2::model::ScalarFormulaRequestType,
    ) -> ScalarFormulaRequest {
        ScalarFormulaRequest { attributes, type_ }
    }
}
