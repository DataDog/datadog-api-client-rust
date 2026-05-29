// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RumPermanentRetentionFilterID {
    RUM_APM_FLAT_SAMPLING,
    SYNTHETICS_SESSIONS,
    FORCED_REPLAY_SESSIONS,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for RumPermanentRetentionFilterID {
    fn to_string(&self) -> String {
        match self {
            Self::RUM_APM_FLAT_SAMPLING => String::from("rum_apm_flat_sampling"),
            Self::SYNTHETICS_SESSIONS => String::from("synthetics_sessions"),
            Self::FORCED_REPLAY_SESSIONS => String::from("forced_replay_sessions"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for RumPermanentRetentionFilterID {
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

impl<'de> Deserialize<'de> for RumPermanentRetentionFilterID {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "rum_apm_flat_sampling" => Self::RUM_APM_FLAT_SAMPLING,
            "synthetics_sessions" => Self::SYNTHETICS_SESSIONS,
            "forced_replay_sessions" => Self::FORCED_REPLAY_SESSIONS,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
