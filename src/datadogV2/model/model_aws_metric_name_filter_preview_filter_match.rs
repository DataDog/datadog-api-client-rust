// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A metric name filter pattern and how many metrics it matched.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AWSMetricNameFilterPreviewFilterMatch {
    /// The number of Datadog metric names matched by this pattern.
    #[serde(rename = "match_count")]
    pub match_count: i64,
    /// The metric name filter pattern.
    #[serde(rename = "pattern")]
    pub pattern: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AWSMetricNameFilterPreviewFilterMatch {
    pub fn new(match_count: i64, pattern: String) -> AWSMetricNameFilterPreviewFilterMatch {
        AWSMetricNameFilterPreviewFilterMatch {
            match_count,
            pattern,
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

impl<'de> Deserialize<'de> for AWSMetricNameFilterPreviewFilterMatch {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AWSMetricNameFilterPreviewFilterMatchVisitor;
        impl<'a> Visitor<'a> for AWSMetricNameFilterPreviewFilterMatchVisitor {
            type Value = AWSMetricNameFilterPreviewFilterMatch;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut match_count: Option<i64> = None;
                let mut pattern: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "match_count" => {
                            match_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "pattern" => {
                            pattern = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let match_count =
                    match_count.ok_or_else(|| M::Error::missing_field("match_count"))?;
                let pattern = pattern.ok_or_else(|| M::Error::missing_field("pattern"))?;

                let content = AWSMetricNameFilterPreviewFilterMatch {
                    match_count,
                    pattern,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AWSMetricNameFilterPreviewFilterMatchVisitor)
    }
}
