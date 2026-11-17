// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// What each recommendation in the digest shows.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CostRecommendationNotificationRuleMessageCustomizationPerRec {
    /// Whether each recommendation shows its effort.
    #[serde(rename = "effort")]
    pub effort: Option<bool>,
    /// Whether each recommendation shows its risk.
    #[serde(rename = "risk")]
    pub risk: Option<bool>,
    /// Whether each recommendation shows why it was made.
    #[serde(rename = "why")]
    pub why: Option<bool>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CostRecommendationNotificationRuleMessageCustomizationPerRec {
    pub fn new() -> CostRecommendationNotificationRuleMessageCustomizationPerRec {
        CostRecommendationNotificationRuleMessageCustomizationPerRec {
            effort: None,
            risk: None,
            why: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn effort(mut self, value: bool) -> Self {
        self.effort = Some(value);
        self
    }

    pub fn risk(mut self, value: bool) -> Self {
        self.risk = Some(value);
        self
    }

    pub fn why(mut self, value: bool) -> Self {
        self.why = Some(value);
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

impl Default for CostRecommendationNotificationRuleMessageCustomizationPerRec {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for CostRecommendationNotificationRuleMessageCustomizationPerRec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CostRecommendationNotificationRuleMessageCustomizationPerRecVisitor;
        impl<'a> Visitor<'a> for CostRecommendationNotificationRuleMessageCustomizationPerRecVisitor {
            type Value = CostRecommendationNotificationRuleMessageCustomizationPerRec;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut effort: Option<bool> = None;
                let mut risk: Option<bool> = None;
                let mut why: Option<bool> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "effort" => {
                            if v.is_null() {
                                continue;
                            }
                            effort = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "risk" => {
                            if v.is_null() {
                                continue;
                            }
                            risk = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "why" => {
                            if v.is_null() {
                                continue;
                            }
                            why = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = CostRecommendationNotificationRuleMessageCustomizationPerRec {
                    effort,
                    risk,
                    why,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer
            .deserialize_any(CostRecommendationNotificationRuleMessageCustomizationPerRecVisitor)
    }
}
