// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The critical asset properties to be updated.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityMonitoringCriticalAssetUpdateAttributes {
    /// Whether the critical asset is enabled.
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    /// The query for the critical asset. It uses the same syntax as the queries to search signals in the Signals Explorer.
    #[serde(rename = "query")]
    pub query: Option<String>,
    /// The rule query of the critical asset, with the same syntax as the search bar for detection rules. This determines which rules this critical asset will apply to.
    #[serde(rename = "rule_query")]
    pub rule_query: Option<String>,
    /// Severity associated with this critical asset. Either an explicit severity can be set, or the severity can be increased or decreased.
    #[serde(rename = "severity")]
    pub severity: Option<crate::datadogV2::model::SecurityMonitoringCriticalAssetSeverity>,
    /// List of tags associated with the critical asset.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// The version of the critical asset being updated. Used for optimistic locking to prevent concurrent modifications.
    #[serde(rename = "version")]
    pub version: Option<i32>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityMonitoringCriticalAssetUpdateAttributes {
    pub fn new() -> SecurityMonitoringCriticalAssetUpdateAttributes {
        SecurityMonitoringCriticalAssetUpdateAttributes {
            enabled: None,
            query: None,
            rule_query: None,
            severity: None,
            tags: None,
            version: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn query(mut self, value: String) -> Self {
        self.query = Some(value);
        self
    }

    pub fn rule_query(mut self, value: String) -> Self {
        self.rule_query = Some(value);
        self
    }

    pub fn severity(
        mut self,
        value: crate::datadogV2::model::SecurityMonitoringCriticalAssetSeverity,
    ) -> Self {
        self.severity = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn version(mut self, value: i32) -> Self {
        self.version = Some(value);
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

impl Default for SecurityMonitoringCriticalAssetUpdateAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SecurityMonitoringCriticalAssetUpdateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityMonitoringCriticalAssetUpdateAttributesVisitor;
        impl<'a> Visitor<'a> for SecurityMonitoringCriticalAssetUpdateAttributesVisitor {
            type Value = SecurityMonitoringCriticalAssetUpdateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut enabled: Option<bool> = None;
                let mut query: Option<String> = None;
                let mut rule_query: Option<String> = None;
                let mut severity: Option<
                    crate::datadogV2::model::SecurityMonitoringCriticalAssetSeverity,
                > = None;
                let mut tags: Option<Vec<String>> = None;
                let mut version: Option<i32> = None;
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
                        "query" => {
                            if v.is_null() {
                                continue;
                            }
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rule_query" => {
                            if v.is_null() {
                                continue;
                            }
                            rule_query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "severity" => {
                            if v.is_null() {
                                continue;
                            }
                            severity = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _severity) = severity {
                                match _severity {
                                    crate::datadogV2::model::SecurityMonitoringCriticalAssetSeverity::UnparsedObject(_severity) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "version" => {
                            if v.is_null() {
                                continue;
                            }
                            version = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SecurityMonitoringCriticalAssetUpdateAttributes {
                    enabled,
                    query,
                    rule_query,
                    severity,
                    tags,
                    version,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityMonitoringCriticalAssetUpdateAttributesVisitor)
    }
}
