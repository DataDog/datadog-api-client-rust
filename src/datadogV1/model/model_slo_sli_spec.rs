// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// A generic SLI specification. This is currently used for time-slice SLOs only.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum SLOSliSpec {
    SLOTimeSliceSpec(Box<crate::datadogV1::model::SLOTimeSliceSpec>),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for SLOSliSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) =
            serde_json::from_value::<Box<crate::datadogV1::model::SLOTimeSliceSpec>>(value.clone())
        {
            if !_v._unparsed {
                return Ok(SLOSliSpec::SLOTimeSliceSpec(_v));
            }
        }

        return Ok(SLOSliSpec::UnparsedObject(crate::datadog::UnparsedObject {
            value,
        }));
    }
}
