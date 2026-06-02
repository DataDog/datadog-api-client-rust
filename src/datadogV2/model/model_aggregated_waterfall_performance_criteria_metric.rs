// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AggregatedWaterfallPerformanceCriteriaMetric {
    LOADING_TIME,
    LARGEST_CONTENTFUL_PAINT,
    FIRST_CONTENTFUL_PAINT,
    INTERACTION_TO_NEXT_PAINT,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for AggregatedWaterfallPerformanceCriteriaMetric {
    fn to_string(&self) -> String {
        match self {
            Self::LOADING_TIME => String::from("loading_time"),
            Self::LARGEST_CONTENTFUL_PAINT => String::from("largest_contentful_paint"),
            Self::FIRST_CONTENTFUL_PAINT => String::from("first_contentful_paint"),
            Self::INTERACTION_TO_NEXT_PAINT => String::from("interaction_to_next_paint"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for AggregatedWaterfallPerformanceCriteriaMetric {
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

impl<'de> Deserialize<'de> for AggregatedWaterfallPerformanceCriteriaMetric {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "loading_time" => Self::LOADING_TIME,
            "largest_contentful_paint" => Self::LARGEST_CONTENTFUL_PAINT,
            "first_contentful_paint" => Self::FIRST_CONTENTFUL_PAINT,
            "interaction_to_next_paint" => Self::INTERACTION_TO_NEXT_PAINT,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
