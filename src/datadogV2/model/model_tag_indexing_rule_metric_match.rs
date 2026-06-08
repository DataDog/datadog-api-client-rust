// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Criteria for matching metrics based on query state.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TagIndexingRuleMetricMatch {
    /// Match metrics that are being queried.
    #[serde(rename = "is_queried")]
    pub is_queried: Option<bool>,
    /// Match metrics that are not being queried.
    #[serde(rename = "not_queried")]
    pub not_queried: Option<bool>,
    /// Match metrics not used in any dashboards or monitors.
    #[serde(rename = "not_used_in_assets")]
    pub not_used_in_assets: Option<bool>,
    /// Window in seconds for evaluating query state.
    #[serde(rename = "queried_window_seconds")]
    pub queried_window_seconds: Option<i64>,
    /// Match metrics used in dashboards or monitors.
    #[serde(rename = "used_in_assets")]
    pub used_in_assets: Option<bool>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TagIndexingRuleMetricMatch {
    pub fn new() -> TagIndexingRuleMetricMatch {
        TagIndexingRuleMetricMatch {
            is_queried: None,
            not_queried: None,
            not_used_in_assets: None,
            queried_window_seconds: None,
            used_in_assets: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn is_queried(mut self, value: bool) -> Self {
        self.is_queried = Some(value);
        self
    }

    pub fn not_queried(mut self, value: bool) -> Self {
        self.not_queried = Some(value);
        self
    }

    pub fn not_used_in_assets(mut self, value: bool) -> Self {
        self.not_used_in_assets = Some(value);
        self
    }

    pub fn queried_window_seconds(mut self, value: i64) -> Self {
        self.queried_window_seconds = Some(value);
        self
    }

    pub fn used_in_assets(mut self, value: bool) -> Self {
        self.used_in_assets = Some(value);
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

impl Default for TagIndexingRuleMetricMatch {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for TagIndexingRuleMetricMatch {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TagIndexingRuleMetricMatchVisitor;
        impl<'a> Visitor<'a> for TagIndexingRuleMetricMatchVisitor {
            type Value = TagIndexingRuleMetricMatch;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut is_queried: Option<bool> = None;
                let mut not_queried: Option<bool> = None;
                let mut not_used_in_assets: Option<bool> = None;
                let mut queried_window_seconds: Option<i64> = None;
                let mut used_in_assets: Option<bool> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "is_queried" => {
                            if v.is_null() {
                                continue;
                            }
                            is_queried = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "not_queried" => {
                            if v.is_null() {
                                continue;
                            }
                            not_queried =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "not_used_in_assets" => {
                            if v.is_null() {
                                continue;
                            }
                            not_used_in_assets =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "queried_window_seconds" => {
                            if v.is_null() {
                                continue;
                            }
                            queried_window_seconds =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "used_in_assets" => {
                            if v.is_null() {
                                continue;
                            }
                            used_in_assets =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = TagIndexingRuleMetricMatch {
                    is_queried,
                    not_queried,
                    not_used_in_assets,
                    queried_window_seconds,
                    used_in_assets,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TagIndexingRuleMetricMatchVisitor)
    }
}
