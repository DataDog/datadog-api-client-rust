// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// A resulting object to put the given computes in over all the matching records.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum CIAppGroupByTotal {
    CIAppGroupByTotalBoolean(bool),
    CIAppGroupByTotalString(String),
    CIAppGroupByTotalNumber(f64),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for CIAppGroupByTotal {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<bool>(value.clone()) {
            return Ok(CIAppGroupByTotal::CIAppGroupByTotalBoolean(_v));
        }
        if let Ok(_v) = serde_json::from_value::<String>(value.clone()) {
            return Ok(CIAppGroupByTotal::CIAppGroupByTotalString(_v));
        }
        if let Ok(_v) = serde_json::from_value::<f64>(value.clone()) {
            return Ok(CIAppGroupByTotal::CIAppGroupByTotalNumber(_v));
        }

        return Ok(CIAppGroupByTotal::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
