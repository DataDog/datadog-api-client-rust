// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExperimentsEVSITrafficUnavailableReason {
    TRAFFIC_UNAVAILABLE,
    INSUFFICIENT_COMPLETE_DAYS,
    NO_TOTAL_EXPOSURES,
    NO_LOOKBACK_EXPOSURES,
    INVALID_TRAFFIC,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for ExperimentsEVSITrafficUnavailableReason {
    fn to_string(&self) -> String {
        match self {
            Self::TRAFFIC_UNAVAILABLE => String::from("traffic_unavailable"),
            Self::INSUFFICIENT_COMPLETE_DAYS => String::from("insufficient_complete_days"),
            Self::NO_TOTAL_EXPOSURES => String::from("no_total_exposures"),
            Self::NO_LOOKBACK_EXPOSURES => String::from("no_lookback_exposures"),
            Self::INVALID_TRAFFIC => String::from("invalid_traffic"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for ExperimentsEVSITrafficUnavailableReason {
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

impl<'de> Deserialize<'de> for ExperimentsEVSITrafficUnavailableReason {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "traffic_unavailable" => Self::TRAFFIC_UNAVAILABLE,
            "insufficient_complete_days" => Self::INSUFFICIENT_COMPLETE_DAYS,
            "no_total_exposures" => Self::NO_TOTAL_EXPOSURES,
            "no_lookback_exposures" => Self::NO_LOOKBACK_EXPOSURES,
            "invalid_traffic" => Self::INVALID_TRAFFIC,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
