// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// The steps used in a Synthetic multi-step API test.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum SyntheticsAPIStep {
    SyntheticsAPITestStep(Box<crate::datadogV1::model::SyntheticsAPITestStep>),
    SyntheticsAPIWaitStep(Box<crate::datadogV1::model::SyntheticsAPIWaitStep>),
    SyntheticsAPISubtestStep(Box<crate::datadogV1::model::SyntheticsAPISubtestStep>),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for SyntheticsAPIStep {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1::model::SyntheticsAPITestStep>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(SyntheticsAPIStep::SyntheticsAPITestStep(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1::model::SyntheticsAPIWaitStep>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(SyntheticsAPIStep::SyntheticsAPIWaitStep(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::SyntheticsAPISubtestStep>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(SyntheticsAPIStep::SyntheticsAPISubtestStep(_v));
            }
        }

        return Ok(SyntheticsAPIStep::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
