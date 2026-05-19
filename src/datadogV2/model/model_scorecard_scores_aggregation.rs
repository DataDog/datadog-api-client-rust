// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ScorecardScoresAggregation {
    BY_ENTITY,
    BY_RULE,
    BY_SCORECARD,
    BY_TEAM,
    BY_KIND,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for ScorecardScoresAggregation {
    fn to_string(&self) -> String {
        match self {
            Self::BY_ENTITY => String::from("by-entity"),
            Self::BY_RULE => String::from("by-rule"),
            Self::BY_SCORECARD => String::from("by-scorecard"),
            Self::BY_TEAM => String::from("by-team"),
            Self::BY_KIND => String::from("by-kind"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for ScorecardScoresAggregation {
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

impl<'de> Deserialize<'de> for ScorecardScoresAggregation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "by-entity" => Self::BY_ENTITY,
            "by-rule" => Self::BY_RULE,
            "by-scorecard" => Self::BY_SCORECARD,
            "by-team" => Self::BY_TEAM,
            "by-kind" => Self::BY_KIND,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
