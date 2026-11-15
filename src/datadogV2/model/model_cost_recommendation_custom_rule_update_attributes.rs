// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes to change. Attributes left out keep their value. `filters` and `settings` replace the stored list
/// and map when sent.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CostRecommendationCustomRuleUpdateAttributes {
    /// Whether the rule is applied when recommendations are generated.
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    /// Tag filters that select the resources the rule applies to. An empty list selects every resource.
    #[serde(rename = "filters")]
    pub filters: Option<Vec<crate::datadogV2::model::CostRecommendationCustomRuleFilter>>,
    /// The name of the rule.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Free-form notes, such as the reason for the customization.
    #[serde(rename = "notes")]
    pub notes: Option<String>,
    /// Setting overrides, keyed by setting name. Settings left out keep the recommender default. The accepted
    /// settings and their ranges depend on the recommender.
    #[serde(rename = "settings")]
    pub settings: Option<std::collections::BTreeMap<String, f64>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CostRecommendationCustomRuleUpdateAttributes {
    pub fn new() -> CostRecommendationCustomRuleUpdateAttributes {
        CostRecommendationCustomRuleUpdateAttributes {
            enabled: None,
            filters: None,
            name: None,
            notes: None,
            settings: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn filters(
        mut self,
        value: Vec<crate::datadogV2::model::CostRecommendationCustomRuleFilter>,
    ) -> Self {
        self.filters = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn notes(mut self, value: String) -> Self {
        self.notes = Some(value);
        self
    }

    pub fn settings(mut self, value: std::collections::BTreeMap<String, f64>) -> Self {
        self.settings = Some(value);
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

impl Default for CostRecommendationCustomRuleUpdateAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for CostRecommendationCustomRuleUpdateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CostRecommendationCustomRuleUpdateAttributesVisitor;
        impl<'a> Visitor<'a> for CostRecommendationCustomRuleUpdateAttributesVisitor {
            type Value = CostRecommendationCustomRuleUpdateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut enabled: Option<bool> = None;
                let mut filters: Option<
                    Vec<crate::datadogV2::model::CostRecommendationCustomRuleFilter>,
                > = None;
                let mut name: Option<String> = None;
                let mut notes: Option<String> = None;
                let mut settings: Option<std::collections::BTreeMap<String, f64>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "filters" => {
                            if v.is_null() {
                                continue;
                            }
                            filters = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "notes" => {
                            if v.is_null() {
                                continue;
                            }
                            notes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "settings" => {
                            if v.is_null() {
                                continue;
                            }
                            settings = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = CostRecommendationCustomRuleUpdateAttributes {
                    enabled,
                    filters,
                    name,
                    notes,
                    settings,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CostRecommendationCustomRuleUpdateAttributesVisitor)
    }
}
