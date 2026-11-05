// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Budget configuration including spend and capacity for the active accounting window.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AiGatewayBudgetListAttributes {
    /// Budget capacity in microcents.
    #[serde(rename = "capacity_microcents")]
    pub capacity_microcents: i64,
    /// Time when the resource was created.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Identifier of the principal that created the resource.
    #[serde(rename = "created_by")]
    pub created_by: String,
    /// Capacity enforced for the active window, in microcents.
    #[serde(rename = "current_period_capacity_microcents")]
    pub current_period_capacity_microcents: i64,
    /// Spend currently counted against the active window, in microcents.
    #[serde(rename = "current_spend_microcents")]
    pub current_spend_microcents: i64,
    /// Whether the resource is enabled.
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// Action taken when spend exceeds the budget.
    #[serde(rename = "enforcement")]
    pub enforcement: crate::datadogV2::model::AiGatewayBudgetEnforcement,
    /// Datadog monitor ID provisioned for this budget. Null when monitor provisioning was skipped or failed.
    #[serialize_always]
    #[serde(rename = "monitor_id")]
    pub monitor_id: Option<i64>,
    /// Human-readable resource name.
    #[serde(rename = "name")]
    pub name: String,
    /// Whether provider or model scope metadata was withheld. Omit unchanged scope fields on an unrelated update to preserve their saved values.
    #[serde(rename = "scope_restricted")]
    pub scope_restricted: bool,
    /// Dimension rules for a template budget. A nonempty selector creates independent counters for matched instances. Omit or pass an empty selector for a legacy fixed-scope budget.
    #[serde(
        rename = "selector",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub selector: Option<Option<Vec<crate::datadogV2::model::AiGatewayBudgetSelectorEntry>>>,
    /// Request tags that must match for the budget to apply.
    #[serde(rename = "tags")]
    pub tags: std::collections::BTreeMap<String, String>,
    /// Optional Datadog Team scope.
    #[serde(rename = "team_id")]
    pub team_id: Option<uuid::Uuid>,
    /// Time when the resource was last updated.
    #[serde(rename = "updated_at")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    /// User scope. Omitted when the budget is not user-scoped.
    #[serde(rename = "user_id")]
    pub user_id: Option<uuid::Uuid>,
    /// Fraction of capacity at which to issue a warning.
    #[serde(rename = "warning_threshold")]
    pub warning_threshold: f64,
    /// End of the active accounting window.
    #[serde(rename = "window_end")]
    pub window_end: chrono::DateTime<chrono::Utc>,
    /// UTC accounting period for a budget.
    #[serde(rename = "window_policy")]
    pub window_policy: crate::datadogV2::model::AiGatewayBudgetWindowPolicy,
    /// Start of the active accounting window.
    #[serde(rename = "window_start")]
    pub window_start: chrono::DateTime<chrono::Utc>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AiGatewayBudgetListAttributes {
    pub fn new(
        capacity_microcents: i64,
        created_at: chrono::DateTime<chrono::Utc>,
        created_by: String,
        current_period_capacity_microcents: i64,
        current_spend_microcents: i64,
        enabled: bool,
        enforcement: crate::datadogV2::model::AiGatewayBudgetEnforcement,
        monitor_id: Option<i64>,
        name: String,
        scope_restricted: bool,
        tags: std::collections::BTreeMap<String, String>,
        updated_at: chrono::DateTime<chrono::Utc>,
        warning_threshold: f64,
        window_end: chrono::DateTime<chrono::Utc>,
        window_policy: crate::datadogV2::model::AiGatewayBudgetWindowPolicy,
        window_start: chrono::DateTime<chrono::Utc>,
    ) -> AiGatewayBudgetListAttributes {
        AiGatewayBudgetListAttributes {
            capacity_microcents,
            created_at,
            created_by,
            current_period_capacity_microcents,
            current_spend_microcents,
            enabled,
            enforcement,
            monitor_id,
            name,
            scope_restricted,
            selector: None,
            tags,
            team_id: None,
            updated_at,
            user_id: None,
            warning_threshold,
            window_end,
            window_policy,
            window_start,
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

    pub fn team_id(mut self, value: uuid::Uuid) -> Self {
        self.team_id = Some(value);
        self
    }

    pub fn user_id(mut self, value: uuid::Uuid) -> Self {
        self.user_id = Some(value);
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

impl<'de> Deserialize<'de> for AiGatewayBudgetListAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AiGatewayBudgetListAttributesVisitor;
        impl<'a> Visitor<'a> for AiGatewayBudgetListAttributesVisitor {
            type Value = AiGatewayBudgetListAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut capacity_microcents: Option<i64> = None;
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut created_by: Option<String> = None;
                let mut current_period_capacity_microcents: Option<i64> = None;
                let mut current_spend_microcents: Option<i64> = None;
                let mut enabled: Option<bool> = None;
                let mut enforcement: Option<crate::datadogV2::model::AiGatewayBudgetEnforcement> =
                    None;
                let mut monitor_id: Option<Option<i64>> = None;
                let mut name: Option<String> = None;
                let mut scope_restricted: Option<bool> = None;
                let mut selector: Option<
                    Option<Vec<crate::datadogV2::model::AiGatewayBudgetSelectorEntry>>,
                > = None;
                let mut tags: Option<std::collections::BTreeMap<String, String>> = None;
                let mut team_id: Option<uuid::Uuid> = None;
                let mut updated_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut user_id: Option<uuid::Uuid> = None;
                let mut warning_threshold: Option<f64> = None;
                let mut window_end: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut window_policy: Option<
                    crate::datadogV2::model::AiGatewayBudgetWindowPolicy,
                > = None;
                let mut window_start: Option<chrono::DateTime<chrono::Utc>> = None;
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
                        "created_at" => {
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_by" => {
                            created_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "current_period_capacity_microcents" => {
                            current_period_capacity_microcents =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "current_spend_microcents" => {
                            current_spend_microcents =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "enabled" => {
                            enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "monitor_id" => {
                            monitor_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "scope_restricted" => {
                            scope_restricted =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "selector" => {
                            selector = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "team_id" => {
                            if v.is_null() {
                                continue;
                            }
                            team_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updated_at" => {
                            updated_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "user_id" => {
                            if v.is_null() {
                                continue;
                            }
                            user_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "warning_threshold" => {
                            warning_threshold =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "window_end" => {
                            window_end = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "window_start" => {
                            window_start =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let created_by = created_by.ok_or_else(|| M::Error::missing_field("created_by"))?;
                let current_period_capacity_microcents = current_period_capacity_microcents
                    .ok_or_else(|| M::Error::missing_field("current_period_capacity_microcents"))?;
                let current_spend_microcents = current_spend_microcents
                    .ok_or_else(|| M::Error::missing_field("current_spend_microcents"))?;
                let enabled = enabled.ok_or_else(|| M::Error::missing_field("enabled"))?;
                let enforcement =
                    enforcement.ok_or_else(|| M::Error::missing_field("enforcement"))?;
                let monitor_id = monitor_id.ok_or_else(|| M::Error::missing_field("monitor_id"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let scope_restricted =
                    scope_restricted.ok_or_else(|| M::Error::missing_field("scope_restricted"))?;
                let tags = tags.ok_or_else(|| M::Error::missing_field("tags"))?;
                let updated_at = updated_at.ok_or_else(|| M::Error::missing_field("updated_at"))?;
                let warning_threshold = warning_threshold
                    .ok_or_else(|| M::Error::missing_field("warning_threshold"))?;
                let window_end = window_end.ok_or_else(|| M::Error::missing_field("window_end"))?;
                let window_policy =
                    window_policy.ok_or_else(|| M::Error::missing_field("window_policy"))?;
                let window_start =
                    window_start.ok_or_else(|| M::Error::missing_field("window_start"))?;

                let content = AiGatewayBudgetListAttributes {
                    capacity_microcents,
                    created_at,
                    created_by,
                    current_period_capacity_microcents,
                    current_spend_microcents,
                    enabled,
                    enforcement,
                    monitor_id,
                    name,
                    scope_restricted,
                    selector,
                    tags,
                    team_id,
                    updated_at,
                    user_id,
                    warning_threshold,
                    window_end,
                    window_policy,
                    window_start,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AiGatewayBudgetListAttributesVisitor)
    }
}
