// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AiGatewayRateLimitInstanceSort {
    UTILIZATION_DESC,
    LAST_SEEN_DESC,
    KEY_ASC,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for AiGatewayRateLimitInstanceSort {
    fn to_string(&self) -> String {
        match self {
            Self::UTILIZATION_DESC => String::from("utilization_desc"),
            Self::LAST_SEEN_DESC => String::from("last_seen_desc"),
            Self::KEY_ASC => String::from("key_asc"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for AiGatewayRateLimitInstanceSort {
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

impl<'de> Deserialize<'de> for AiGatewayRateLimitInstanceSort {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "utilization_desc" => Self::UTILIZATION_DESC,
            "last_seen_desc" => Self::LAST_SEEN_DESC,
            "key_asc" => Self::KEY_ASC,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
