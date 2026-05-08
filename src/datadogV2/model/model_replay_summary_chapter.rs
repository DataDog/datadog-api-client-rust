// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A chapter within a RUM replay summary, representing a distinct segment of user activity.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ReplaySummaryChapter {
    /// End time of the chapter in milliseconds.
    #[serde(rename = "end_ms")]
    pub end_ms: i64,
    /// Start time of the chapter in milliseconds.
    #[serde(rename = "start_ms")]
    pub start_ms: i64,
    /// Description of user activity during this chapter.
    #[serde(rename = "text")]
    pub text: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ReplaySummaryChapter {
    pub fn new(end_ms: i64, start_ms: i64, text: String) -> ReplaySummaryChapter {
        ReplaySummaryChapter {
            end_ms,
            start_ms,
            text,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for ReplaySummaryChapter {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ReplaySummaryChapterVisitor;
        impl<'a> Visitor<'a> for ReplaySummaryChapterVisitor {
            type Value = ReplaySummaryChapter;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut end_ms: Option<i64> = None;
                let mut start_ms: Option<i64> = None;
                let mut text: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "end_ms" => {
                            end_ms = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "start_ms" => {
                            start_ms = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "text" => {
                            text = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let end_ms = end_ms.ok_or_else(|| M::Error::missing_field("end_ms"))?;
                let start_ms = start_ms.ok_or_else(|| M::Error::missing_field("start_ms"))?;
                let text = text.ok_or_else(|| M::Error::missing_field("text"))?;

                let content = ReplaySummaryChapter {
                    end_ms,
                    start_ms,
                    text,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ReplaySummaryChapterVisitor)
    }
}
