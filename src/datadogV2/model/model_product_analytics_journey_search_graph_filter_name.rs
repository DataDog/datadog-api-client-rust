// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProductAnalyticsJourneySearchGraphFilterName {
    TIME_TO_CONVERT,
    SESSION,
    DROPOFF_RATE,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for ProductAnalyticsJourneySearchGraphFilterName {
    fn to_string(&self) -> String {
        match self {
            Self::TIME_TO_CONVERT => String::from("__dd.time_to_convert"),
            Self::SESSION => String::from("__dd.session"),
            Self::DROPOFF_RATE => String::from("__dd.dropoff_rate"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for ProductAnalyticsJourneySearchGraphFilterName {
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

impl<'de> Deserialize<'de> for ProductAnalyticsJourneySearchGraphFilterName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "__dd.time_to_convert" => Self::TIME_TO_CONVERT,
            "__dd.session" => Self::SESSION,
            "__dd.dropoff_rate" => Self::DROPOFF_RATE,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
