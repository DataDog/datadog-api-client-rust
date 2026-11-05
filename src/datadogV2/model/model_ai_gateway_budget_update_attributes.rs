// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Mutable attributes of an AI Gateway budget.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AiGatewayBudgetUpdateAttributes {
    /// Whether a capacity change also applies to the active accounting period. Requires capacity_microcents. Unsupported for template budgets.
    #[serde(rename = "apply_to_current_period")]
    pub apply_to_current_period: Option<bool>,
    /// Default capacity for future accounting periods, in microcents.
    #[serde(rename = "capacity_microcents")]
    pub capacity_microcents: Option<i64>,
    /// Whether the budget is enabled.
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    /// Action taken when spend exceeds the budget.
    #[serde(rename = "enforcement")]
    pub enforcement: Option<crate::datadogV2::model::AiGatewayBudgetEnforcement>,
    /// Human-readable resource name.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Optional reason recorded with an active-period capacity adjustment.
    #[serde(rename = "reason")]
    pub reason: Option<String>,
    /// Initialize current-period spend from historical priced LLM Observability spans. Defaults to false. Unsupported for template budgets; failures can be retried with an update.
    #[serde(rename = "track_existing")]
    pub track_existing: Option<bool>,
    /// Fraction of capacity at which to issue a warning.
    #[serde(rename = "warning_threshold")]
    pub warning_threshold: Option<f64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AiGatewayBudgetUpdateAttributes {
    pub fn new() -> AiGatewayBudgetUpdateAttributes {
        AiGatewayBudgetUpdateAttributes {
            apply_to_current_period: None,
            capacity_microcents: None,
            enabled: None,
            enforcement: None,
            name: None,
            reason: None,
            track_existing: None,
            warning_threshold: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn apply_to_current_period(mut self, value: bool) -> Self {
        self.apply_to_current_period = Some(value);
        self
    }

    pub fn capacity_microcents(mut self, value: i64) -> Self {
        self.capacity_microcents = Some(value);
        self
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn enforcement(
        mut self,
        value: crate::datadogV2::model::AiGatewayBudgetEnforcement,
    ) -> Self {
        self.enforcement = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn reason(mut self, value: String) -> Self {
        self.reason = Some(value);
        self
    }

    pub fn track_existing(mut self, value: bool) -> Self {
        self.track_existing = Some(value);
        self
    }

    pub fn warning_threshold(mut self, value: f64) -> Self {
        self.warning_threshold = Some(value);
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

impl Default for AiGatewayBudgetUpdateAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for AiGatewayBudgetUpdateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AiGatewayBudgetUpdateAttributesVisitor;
        impl<'a> Visitor<'a> for AiGatewayBudgetUpdateAttributesVisitor {
            type Value = AiGatewayBudgetUpdateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut apply_to_current_period: Option<bool> = None;
                let mut capacity_microcents: Option<i64> = None;
                let mut enabled: Option<bool> = None;
                let mut enforcement: Option<crate::datadogV2::model::AiGatewayBudgetEnforcement> =
                    None;
                let mut name: Option<String> = None;
                let mut reason: Option<String> = None;
                let mut track_existing: Option<bool> = None;
                let mut warning_threshold: Option<f64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "apply_to_current_period" => {
                            if v.is_null() {
                                continue;
                            }
                            apply_to_current_period =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "capacity_microcents" => {
                            if v.is_null() {
                                continue;
                            }
                            capacity_microcents =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "enforcement" => {
                            if v.is_null() {
                                continue;
                            }
                            enforcement =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _enforcement) = enforcement {
                                match _enforcement {
                                    crate::datadogV2::model::AiGatewayBudgetEnforcement::UnparsedObject(_enforcement) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "reason" => {
                            if v.is_null() {
                                continue;
                            }
                            reason = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "track_existing" => {
                            if v.is_null() {
                                continue;
                            }
                            track_existing =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "warning_threshold" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            warning_threshold =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = AiGatewayBudgetUpdateAttributes {
                    apply_to_current_period,
                    capacity_microcents,
                    enabled,
                    enforcement,
                    name,
                    reason,
                    track_existing,
                    warning_threshold,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AiGatewayBudgetUpdateAttributesVisitor)
    }
}
