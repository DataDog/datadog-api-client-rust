// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// The value to use for logs that don't have the facet used to group by.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum RUMGroupByMissing {
    RUMGroupByMissingString(String),
    RUMGroupByMissingNumber(f64),
    UnparsedObject(crate::datadog::UnparsedObejct),
}

impl<'de> Deserialize<'de> for RUMGroupByMissing {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<String>(value.clone()) {
            return Ok(RUMGroupByMissing::RUMGroupByMissingString(_v));
        }
        if let Ok(_v) = serde_json::from_value::<f64>(value.clone()) {
            return Ok(RUMGroupByMissing::RUMGroupByMissingNumber(_v));
        }

        return Ok(RUMGroupByMissing::UnparsedObject(
            crate::datadog::UnparsedObejct { value },
        ));
    }
}
