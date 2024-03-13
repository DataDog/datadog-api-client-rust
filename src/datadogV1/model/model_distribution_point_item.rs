// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// List of distribution point.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum DistributionPointItem {
    DistributionPointTimestamp(f64),
    DistributionPointData(Vec<f64>),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for DistributionPointItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<f64>(value.clone()) {
            return Ok(DistributionPointItem::DistributionPointTimestamp(_v));
        }
        if let Ok(_v) = serde_json::from_value::<Vec<f64>>(value.clone()) {
            return Ok(DistributionPointItem::DistributionPointData(_v));
        }

        return Ok(DistributionPointItem::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
