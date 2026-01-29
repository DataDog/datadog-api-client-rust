// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Auto-transition assigned cases settings
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AutoTransitionAssignedCases {
    /// Whether to auto-transition cases when self-assigned
    #[serde(rename = "auto_transition_assigned_cases_on_self_assigned")]
    pub auto_transition_assigned_cases_on_self_assigned: Option<bool>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AutoTransitionAssignedCases {
    pub fn new() -> AutoTransitionAssignedCases {
        AutoTransitionAssignedCases {
            auto_transition_assigned_cases_on_self_assigned: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn auto_transition_assigned_cases_on_self_assigned(mut self, value: bool) -> Self {
        self.auto_transition_assigned_cases_on_self_assigned = Some(value);
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

impl Default for AutoTransitionAssignedCases {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for AutoTransitionAssignedCases {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AutoTransitionAssignedCasesVisitor;
        impl<'a> Visitor<'a> for AutoTransitionAssignedCasesVisitor {
            type Value = AutoTransitionAssignedCases;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut auto_transition_assigned_cases_on_self_assigned: Option<bool> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "auto_transition_assigned_cases_on_self_assigned" => {
                            if v.is_null() {
                                continue;
                            }
                            auto_transition_assigned_cases_on_self_assigned =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = AutoTransitionAssignedCases {
                    auto_transition_assigned_cases_on_self_assigned,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AutoTransitionAssignedCasesVisitor)
    }
}
