// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a tag policy score.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TagPolicyScoreAttributesResponse {
    /// The compliance score for the tag policy.
    #[serde(rename = "score")]
    pub score: f64,
    /// End timestamp for the score calculation period.
    #[serde(rename = "ts_end")]
    pub ts_end: i64,
    /// Start timestamp for the score calculation period.
    #[serde(rename = "ts_start")]
    pub ts_start: i64,
    /// The version of the score.
    #[serde(rename = "version")]
    pub version: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TagPolicyScoreAttributesResponse {
    pub fn new(score: f64, ts_end: i64, ts_start: i64) -> TagPolicyScoreAttributesResponse {
        TagPolicyScoreAttributesResponse {
            score,
            ts_end,
            ts_start,
            version: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn version(mut self, value: i64) -> Self {
        self.version = Some(value);
        self
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for TagPolicyScoreAttributesResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TagPolicyScoreAttributesResponseVisitor;
        impl<'a> Visitor<'a> for TagPolicyScoreAttributesResponseVisitor {
            type Value = TagPolicyScoreAttributesResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut score: Option<f64> = None;
                let mut ts_end: Option<i64> = None;
                let mut ts_start: Option<i64> = None;
                let mut version: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "score" => {
                            score = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ts_end" => {
                            ts_end = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ts_start" => {
                            ts_start = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "version" => {
                            if v.is_null() {
                                continue;
                            }
                            version = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let score = score.ok_or_else(|| M::Error::missing_field("score"))?;
                let ts_end = ts_end.ok_or_else(|| M::Error::missing_field("ts_end"))?;
                let ts_start = ts_start.ok_or_else(|| M::Error::missing_field("ts_start"))?;

                let content = TagPolicyScoreAttributesResponse {
                    score,
                    ts_end,
                    ts_start,
                    version,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TagPolicyScoreAttributesResponseVisitor)
    }
}
