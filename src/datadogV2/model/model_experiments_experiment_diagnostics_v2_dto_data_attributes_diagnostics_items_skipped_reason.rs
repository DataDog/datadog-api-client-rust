// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExperimentsExperimentDiagnosticsV2DTODataAttributesDiagnosticsItemsSkippedReason {
    NO_ASSIGNMENTS,
    NO_DIMENSIONAL_DATA,
    NO_METRIC_DATA,
    ZERO_VARIANCE,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for ExperimentsExperimentDiagnosticsV2DTODataAttributesDiagnosticsItemsSkippedReason {
    fn to_string(&self) -> String {
        match self {
            Self::NO_ASSIGNMENTS => String::from("NO_ASSIGNMENTS"),
            Self::NO_DIMENSIONAL_DATA => String::from("NO_DIMENSIONAL_DATA"),
            Self::NO_METRIC_DATA => String::from("NO_METRIC_DATA"),
            Self::ZERO_VARIANCE => String::from("ZERO_VARIANCE"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize
    for ExperimentsExperimentDiagnosticsV2DTODataAttributesDiagnosticsItemsSkippedReason
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::UnparsedObject(v) => v.serialize(serializer),
            _ => serializer.serialize_str(self.to_string().as_str()),
        }
    }
}

impl<'de> Deserialize<'de>
    for ExperimentsExperimentDiagnosticsV2DTODataAttributesDiagnosticsItemsSkippedReason
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "NO_ASSIGNMENTS" => Self::NO_ASSIGNMENTS,
            "NO_DIMENSIONAL_DATA" => Self::NO_DIMENSIONAL_DATA,
            "NO_METRIC_DATA" => Self::NO_METRIC_DATA,
            "ZERO_VARIANCE" => Self::ZERO_VARIANCE,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
