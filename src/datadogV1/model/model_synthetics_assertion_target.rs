// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// An assertion which uses a simple target.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsAssertionTarget {
    /// Assertion operator to apply.
    #[serde(rename = "operator")]
    pub operator: crate::datadogV1::model::SyntheticsAssertionOperator,
    /// The associated assertion property.
    #[serde(rename = "property")]
    pub property: Option<String>,
    /// Value used by the operator.
    #[serde(rename = "target")]
    pub target: serde_json::Value,
    /// Timings scope for response time assertions.
    #[serde(rename = "timingsScope")]
    pub timings_scope: Option<crate::datadogV1::model::SyntheticsAssertionTimingsScope>,
    /// Type of the assertion.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::SyntheticsAssertionType,
}

impl SyntheticsAssertionTarget {
    pub fn new(
        operator: crate::datadogV1::model::SyntheticsAssertionOperator,
        target: serde_json::Value,
        type_: crate::datadogV1::model::SyntheticsAssertionType,
    ) -> SyntheticsAssertionTarget {
        SyntheticsAssertionTarget {
            operator,
            property: None,
            target,
            timings_scope: None,
            type_,
        }
    }

    pub fn property(&mut self, value: String) -> &mut Self {
        self.property = Some(value);
        self
    }

    pub fn timings_scope(
        &mut self,
        value: crate::datadogV1::model::SyntheticsAssertionTimingsScope,
    ) -> &mut Self {
        self.timings_scope = Some(value);
        self
    }
}
