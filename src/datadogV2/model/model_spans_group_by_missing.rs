// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// The value to use for spans that don't have the facet used to group by.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum SpansGroupByMissing {
    SpansGroupByMissingString(String),
    SpansGroupByMissingNumber(f64),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for SpansGroupByMissing {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<String>(value.clone()) {
            return Ok(SpansGroupByMissing::SpansGroupByMissingString(_v));
        }
        if let Ok(_v) = serde_json::from_value::<f64>(value.clone()) {
            return Ok(SpansGroupByMissing::SpansGroupByMissingNumber(_v));
        }

        return Ok(SpansGroupByMissing::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
