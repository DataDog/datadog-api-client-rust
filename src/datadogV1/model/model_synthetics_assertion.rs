// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Object describing the assertions type, their associated operator,
/// which property they apply, and upon which target.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum SyntheticsAssertion {
    SyntheticsAssertionTarget(Box<crate::datadogV1::model::SyntheticsAssertionTarget>),
    SyntheticsAssertionJSONPathTarget(
        Box<crate::datadogV1::model::SyntheticsAssertionJSONPathTarget>,
    ),
    SyntheticsAssertionXPathTarget(Box<crate::datadogV1::model::SyntheticsAssertionXPathTarget>),
    UnparsedObject(crate::datadog::UnparsedObejct),
}

impl<'de> Deserialize<'de> for SyntheticsAssertion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::SyntheticsAssertionTarget>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(SyntheticsAssertion::SyntheticsAssertionTarget(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::SyntheticsAssertionJSONPathTarget>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(SyntheticsAssertion::SyntheticsAssertionJSONPathTarget(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::SyntheticsAssertionXPathTarget>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(SyntheticsAssertion::SyntheticsAssertionXPathTarget(_v));
            }
        }

        return Ok(SyntheticsAssertion::UnparsedObject(
            crate::datadog::UnparsedObejct { value },
        ));
    }
}
