// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Response for bulk deleting security monitoring rules.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityMonitoringRuleBulkDeleteResponse {
    /// List of successfully deleted rule IDs.
    #[serde(rename = "deletedRules")]
    pub deleted_rules: Option<Vec<String>>,
    /// List of rule IDs that could not be deleted.
    #[serde(rename = "failedRules")]
    pub failed_rules: Option<Vec<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityMonitoringRuleBulkDeleteResponse {
    pub fn new() -> SecurityMonitoringRuleBulkDeleteResponse {
        SecurityMonitoringRuleBulkDeleteResponse {
            deleted_rules: None,
            failed_rules: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn deleted_rules(mut self, value: Vec<String>) -> Self {
        self.deleted_rules = Some(value);
        self
    }

    pub fn failed_rules(mut self, value: Vec<String>) -> Self {
        self.failed_rules = Some(value);
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

impl Default for SecurityMonitoringRuleBulkDeleteResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SecurityMonitoringRuleBulkDeleteResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityMonitoringRuleBulkDeleteResponseVisitor;
        impl<'a> Visitor<'a> for SecurityMonitoringRuleBulkDeleteResponseVisitor {
            type Value = SecurityMonitoringRuleBulkDeleteResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut deleted_rules: Option<Vec<String>> = None;
                let mut failed_rules: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "deletedRules" => {
                            if v.is_null() {
                                continue;
                            }
                            deleted_rules =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "failedRules" => {
                            if v.is_null() {
                                continue;
                            }
                            failed_rules =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SecurityMonitoringRuleBulkDeleteResponse {
                    deleted_rules,
                    failed_rules,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityMonitoringRuleBulkDeleteResponseVisitor)
    }
}
