// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// An assertion for the `validatesXPath` operator.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsAssertionXPathTarget {
    /// Assertion operator to apply.
    #[serde(rename = "operator")]
    pub operator: crate::datadogV1::model::SyntheticsAssertionXPathOperator,
    /// The associated assertion property.
    #[serde(rename = "property")]
    pub property: Option<String>,
    /// Composed target for `validatesXPath` operator.
    #[serde(rename = "target")]
    pub target: Option<crate::datadogV1::model::SyntheticsAssertionXPathTargetTarget>,
    /// Type of the assertion.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::SyntheticsAssertionType,
}

impl SyntheticsAssertionXPathTarget {
    pub fn new(
        operator: crate::datadogV1::model::SyntheticsAssertionXPathOperator,
        type_: crate::datadogV1::model::SyntheticsAssertionType,
    ) -> SyntheticsAssertionXPathTarget {
        SyntheticsAssertionXPathTarget {
            operator,
            property: None,
            target: None,
            type_,
        }
    }

    pub fn property(mut self, value: String) -> Self {
        self.property = Some(value);
        self
    }

    pub fn target(
        mut self,
        value: crate::datadogV1::model::SyntheticsAssertionXPathTargetTarget,
    ) -> Self {
        self.target = Some(value);
        self
    }
}
