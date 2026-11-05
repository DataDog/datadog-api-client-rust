// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes used to create an AI Gateway budget.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AiGatewayBudgetCreateAttributes {
    /// Budget capacity in microcents.
    #[serde(rename = "capacity_microcents")]
    pub capacity_microcents: i64,
    /// Action taken when spend exceeds the budget.
    #[serde(rename = "enforcement")]
    pub enforcement: crate::datadogV2::model::AiGatewayBudgetEnforcement,
    /// Human-readable resource name.
    #[serde(rename = "name")]
    pub name: String,
    /// Dimension rules for a template budget. A nonempty selector creates independent counters for matched instances. Omit or pass an empty selector for a legacy fixed-scope budget.
    #[serde(
        rename = "selector",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub selector: Option<Option<Vec<crate::datadogV2::model::AiGatewayBudgetSelectorEntry>>>,
    /// Request tags that must match for the budget to apply.
    #[serde(rename = "tags")]
    pub tags: Option<std::collections::BTreeMap<String, String>>,
    /// Optional Datadog Team scope.
    #[serde(rename = "team_id")]
    pub team_id: Option<uuid::Uuid>,
    /// Initialize current-period spend from historical priced LLM Observability spans. Defaults to false. Unsupported for template budgets; failures can be retried with an update.
    #[serde(rename = "track_existing")]
    pub track_existing: Option<bool>,
    /// Optional user scope.
    #[serde(rename = "user_id")]
    pub user_id: Option<uuid::Uuid>,
    /// Fraction of capacity at which to issue a warning.
    #[serde(rename = "warning_threshold")]
    pub warning_threshold: Option<f64>,
    /// UTC accounting period for a budget.
    #[serde(rename = "window_policy")]
    pub window_policy: crate::datadogV2::model::AiGatewayBudgetWindowPolicy,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AiGatewayBudgetCreateAttributes {
    pub fn new(
        capacity_microcents: i64,
        enforcement: crate::datadogV2::model::AiGatewayBudgetEnforcement,
        name: String,
        window_policy: crate::datadogV2::model::AiGatewayBudgetWindowPolicy,
    ) -> AiGatewayBudgetCreateAttributes {
        AiGatewayBudgetCreateAttributes {
            capacity_microcents,
            enforcement,
            name,
            selector: None,
            tags: None,
            team_id: None,
            track_existing: None,
            user_id: None,
            warning_threshold: None,
            window_policy,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn selector(
        mut self,
        value: Option<Vec<crate::datadogV2::model::AiGatewayBudgetSelectorEntry>>,
    ) -> Self {
        self.selector = Some(value);
        self
    }

    pub fn tags(mut self, value: std::collections::BTreeMap<String, String>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn team_id(mut self, value: uuid::Uuid) -> Self {
        self.team_id = Some(value);
        self
    }

    pub fn track_existing(mut self, value: bool) -> Self {
        self.track_existing = Some(value);
        self
    }

    pub fn user_id(mut self, value: uuid::Uuid) -> Self {
        self.user_id = Some(value);
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

impl<'de> Deserialize<'de> for AiGatewayBudgetCreateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AiGatewayBudgetCreateAttributesVisitor;
        impl<'a> Visitor<'a> for AiGatewayBudgetCreateAttributesVisitor {
            type Value = AiGatewayBudgetCreateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut capacity_microcents: Option<i64> = None;
                let mut enforcement: Option<crate::datadogV2::model::AiGatewayBudgetEnforcement> =
                    None;
                let mut name: Option<String> = None;
                let mut selector: Option<
                    Option<Vec<crate::datadogV2::model::AiGatewayBudgetSelectorEntry>>,
                > = None;
                let mut tags: Option<std::collections::BTreeMap<String, String>> = None;
                let mut team_id: Option<uuid::Uuid> = None;
                let mut track_existing: Option<bool> = None;
                let mut user_id: Option<uuid::Uuid> = None;
                let mut warning_threshold: Option<f64> = None;
                let mut window_policy: Option<
                    crate::datadogV2::model::AiGatewayBudgetWindowPolicy,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "capacity_microcents" => {
                            capacity_microcents =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "enforcement" => {
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
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "selector" => {
                            selector = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "team_id" => {
                            if v.is_null() {
                                continue;
                            }
                            team_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "track_existing" => {
                            if v.is_null() {
                                continue;
                            }
                            track_existing =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "user_id" => {
                            if v.is_null() {
                                continue;
                            }
                            user_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "warning_threshold" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            warning_threshold =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "window_policy" => {
                            window_policy =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _window_policy) = window_policy {
                                match _window_policy {
                                    crate::datadogV2::model::AiGatewayBudgetWindowPolicy::UnparsedObject(_window_policy) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let capacity_microcents = capacity_microcents
                    .ok_or_else(|| M::Error::missing_field("capacity_microcents"))?;
                let enforcement =
                    enforcement.ok_or_else(|| M::Error::missing_field("enforcement"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let window_policy =
                    window_policy.ok_or_else(|| M::Error::missing_field("window_policy"))?;

                let content = AiGatewayBudgetCreateAttributes {
                    capacity_microcents,
                    enforcement,
                    name,
                    selector,
                    tags,
                    team_id,
                    track_existing,
                    user_id,
                    warning_threshold,
                    window_policy,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AiGatewayBudgetCreateAttributesVisitor)
    }
}
