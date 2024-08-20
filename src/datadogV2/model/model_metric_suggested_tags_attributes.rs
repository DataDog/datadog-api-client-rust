// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object containing the definition of a metric's actively queried tags and aggregations.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MetricSuggestedTagsAttributes {
    /// List of aggregation combinations that have been actively queried.
    #[serde(rename = "active_aggregations")]
    pub active_aggregations: Option<Vec<crate::datadogV2::model::MetricCustomAggregation>>,
    /// List of tag keys that have been actively queried.
    #[serde(rename = "active_tags")]
    pub active_tags: Option<Vec<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MetricSuggestedTagsAttributes {
    pub fn new() -> MetricSuggestedTagsAttributes {
        MetricSuggestedTagsAttributes {
            active_aggregations: None,
            active_tags: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn active_aggregations(
        mut self,
        value: Vec<crate::datadogV2::model::MetricCustomAggregation>,
    ) -> Self {
        self.active_aggregations = Some(value);
        self
    }

    pub fn active_tags(mut self, value: Vec<String>) -> Self {
        self.active_tags = Some(value);
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

impl Default for MetricSuggestedTagsAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for MetricSuggestedTagsAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MetricSuggestedTagsAttributesVisitor;
        impl<'a> Visitor<'a> for MetricSuggestedTagsAttributesVisitor {
            type Value = MetricSuggestedTagsAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut active_aggregations: Option<
                    Vec<crate::datadogV2::model::MetricCustomAggregation>,
                > = None;
                let mut active_tags: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "active_aggregations" => {
                            if v.is_null() {
                                continue;
                            }
                            active_aggregations =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "active_tags" => {
                            if v.is_null() {
                                continue;
                            }
                            active_tags =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = MetricSuggestedTagsAttributes {
                    active_aggregations,
                    active_tags,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MetricSuggestedTagsAttributesVisitor)
    }
}
