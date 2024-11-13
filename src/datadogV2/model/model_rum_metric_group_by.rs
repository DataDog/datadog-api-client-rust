// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A group by rule.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RumMetricGroupBy {
    /// The path to the value the rum-based metric will be aggregated over.
    #[serde(rename = "path")]
    pub path: String,
    /// Eventual name of the tag that gets created. By default, `path` is used as the tag name.
    #[serde(rename = "tag_name")]
    pub tag_name: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RumMetricGroupBy {
    pub fn new(path: String) -> RumMetricGroupBy {
        RumMetricGroupBy {
            path,
            tag_name: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn tag_name(mut self, value: String) -> Self {
        self.tag_name = Some(value);
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

impl<'de> Deserialize<'de> for RumMetricGroupBy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RumMetricGroupByVisitor;
        impl<'a> Visitor<'a> for RumMetricGroupByVisitor {
            type Value = RumMetricGroupBy;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut path: Option<String> = None;
                let mut tag_name: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "path" => {
                            path = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tag_name" => {
                            if v.is_null() {
                                continue;
                            }
                            tag_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let path = path.ok_or_else(|| M::Error::missing_field("path"))?;

                let content = RumMetricGroupBy {
                    path,
                    tag_name,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RumMetricGroupByVisitor)
    }
}