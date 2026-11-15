// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Where a message action appears.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CostRecommendationNotificationRuleMessageAction {
    /// Whether the action appears once for the whole digest.
    #[serde(rename = "bulk")]
    pub bulk: Option<bool>,
    /// Whether the action appears on each recommendation.
    #[serde(rename = "per_rec")]
    pub per_rec: Option<bool>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CostRecommendationNotificationRuleMessageAction {
    pub fn new() -> CostRecommendationNotificationRuleMessageAction {
        CostRecommendationNotificationRuleMessageAction {
            bulk: None,
            per_rec: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn bulk(mut self, value: bool) -> Self {
        self.bulk = Some(value);
        self
    }

    pub fn per_rec(mut self, value: bool) -> Self {
        self.per_rec = Some(value);
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

impl Default for CostRecommendationNotificationRuleMessageAction {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for CostRecommendationNotificationRuleMessageAction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CostRecommendationNotificationRuleMessageActionVisitor;
        impl<'a> Visitor<'a> for CostRecommendationNotificationRuleMessageActionVisitor {
            type Value = CostRecommendationNotificationRuleMessageAction;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut bulk: Option<bool> = None;
                let mut per_rec: Option<bool> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "bulk" => {
                            if v.is_null() {
                                continue;
                            }
                            bulk = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "per_rec" => {
                            if v.is_null() {
                                continue;
                            }
                            per_rec = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = CostRecommendationNotificationRuleMessageAction {
                    bulk,
                    per_rec,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CostRecommendationNotificationRuleMessageActionVisitor)
    }
}
