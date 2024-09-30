// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Reference table for the rule.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityMonitoringReferenceTable {
    /// Whether to include or exclude the matched values.
    #[serde(rename = "checkPresence")]
    pub check_presence: Option<bool>,
    /// The name of the column in the reference table.
    #[serde(rename = "columnName")]
    pub column_name: Option<String>,
    /// The field in the log to match against the reference table.
    #[serde(rename = "logFieldPath")]
    pub log_field_path: Option<String>,
    /// The name of the rule query to apply the reference table to.
    #[serde(rename = "ruleQueryName")]
    pub rule_query_name: Option<String>,
    /// The name of the reference table.
    #[serde(rename = "tableName")]
    pub table_name: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityMonitoringReferenceTable {
    pub fn new() -> SecurityMonitoringReferenceTable {
        SecurityMonitoringReferenceTable {
            check_presence: None,
            column_name: None,
            log_field_path: None,
            rule_query_name: None,
            table_name: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn check_presence(mut self, value: bool) -> Self {
        self.check_presence = Some(value);
        self
    }

    pub fn column_name(mut self, value: String) -> Self {
        self.column_name = Some(value);
        self
    }

    pub fn log_field_path(mut self, value: String) -> Self {
        self.log_field_path = Some(value);
        self
    }

    pub fn rule_query_name(mut self, value: String) -> Self {
        self.rule_query_name = Some(value);
        self
    }

    pub fn table_name(mut self, value: String) -> Self {
        self.table_name = Some(value);
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

impl Default for SecurityMonitoringReferenceTable {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SecurityMonitoringReferenceTable {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityMonitoringReferenceTableVisitor;
        impl<'a> Visitor<'a> for SecurityMonitoringReferenceTableVisitor {
            type Value = SecurityMonitoringReferenceTable;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut check_presence: Option<bool> = None;
                let mut column_name: Option<String> = None;
                let mut log_field_path: Option<String> = None;
                let mut rule_query_name: Option<String> = None;
                let mut table_name: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "checkPresence" => {
                            if v.is_null() {
                                continue;
                            }
                            check_presence =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "columnName" => {
                            if v.is_null() {
                                continue;
                            }
                            column_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "logFieldPath" => {
                            if v.is_null() {
                                continue;
                            }
                            log_field_path =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ruleQueryName" => {
                            if v.is_null() {
                                continue;
                            }
                            rule_query_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tableName" => {
                            if v.is_null() {
                                continue;
                            }
                            table_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SecurityMonitoringReferenceTable {
                    check_presence,
                    column_name,
                    log_field_path,
                    rule_query_name,
                    table_name,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityMonitoringReferenceTableVisitor)
    }
}
