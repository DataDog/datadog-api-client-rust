// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Request-only confidence threshold override. A number sets the threshold
/// for every threat category; an object supplies a global threshold and
/// optional per-category overrides. Omit this field to use the stored
/// sensitivity policy. This field is not returned in evaluation responses.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum AIGuardConfidenceThresholdOverride {
    AIGuardGlobalConfidenceThreshold(f64),
    AIGuardConfidenceThreshold(Box<crate::datadogV2::model::AIGuardConfidenceThreshold>),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for AIGuardConfidenceThresholdOverride {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<f64>(value.clone()) {
            return Ok(AIGuardConfidenceThresholdOverride::AIGuardGlobalConfidenceThreshold(_v));
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::AIGuardConfidenceThreshold>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(AIGuardConfidenceThresholdOverride::AIGuardConfidenceThreshold(_v));
            }
        }

        return Ok(AIGuardConfidenceThresholdOverride::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
