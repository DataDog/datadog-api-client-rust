// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The attributes of the critical asset.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityMonitoringCriticalAssetAttributes {
    /// ID of user who created the critical asset.
    #[serde(rename = "creation_author_id")]
    pub creation_author_id: Option<i64>,
    /// A Unix millisecond timestamp given the creation date of the critical asset.
    #[serde(rename = "creation_date")]
    pub creation_date: Option<i64>,
    /// A user.
    #[serde(rename = "creator")]
    pub creator: Option<crate::datadogV2::model::SecurityMonitoringUser>,
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
    /// ID of user who updated the critical asset.
    #[serde(rename = "update_author_id")]
    pub update_author_id: Option<i64>,
    /// A Unix millisecond timestamp given the update date of the critical asset.
    #[serde(rename = "update_date")]
    pub update_date: Option<i64>,
    /// A user.
    #[serde(rename = "updater")]
    pub updater: Option<crate::datadogV2::model::SecurityMonitoringUser>,
    /// The version of the critical asset; it starts at 1, and is incremented at each update.
    #[serde(rename = "version")]
    pub version: Option<i32>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityMonitoringCriticalAssetAttributes {
    pub fn new() -> SecurityMonitoringCriticalAssetAttributes {
        SecurityMonitoringCriticalAssetAttributes {
            creation_author_id: None,
            creation_date: None,
            creator: None,
            enabled: None,
            query: None,
            rule_query: None,
            severity: None,
            tags: None,
            update_author_id: None,
            update_date: None,
            updater: None,
            version: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn creation_author_id(mut self, value: i64) -> Self {
        self.creation_author_id = Some(value);
        self
    }

    pub fn creation_date(mut self, value: i64) -> Self {
        self.creation_date = Some(value);
        self
    }

    pub fn creator(mut self, value: crate::datadogV2::model::SecurityMonitoringUser) -> Self {
        self.creator = Some(value);
        self
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

    pub fn update_author_id(mut self, value: i64) -> Self {
        self.update_author_id = Some(value);
        self
    }

    pub fn update_date(mut self, value: i64) -> Self {
        self.update_date = Some(value);
        self
    }

    pub fn updater(mut self, value: crate::datadogV2::model::SecurityMonitoringUser) -> Self {
        self.updater = Some(value);
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

impl Default for SecurityMonitoringCriticalAssetAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SecurityMonitoringCriticalAssetAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityMonitoringCriticalAssetAttributesVisitor;
        impl<'a> Visitor<'a> for SecurityMonitoringCriticalAssetAttributesVisitor {
            type Value = SecurityMonitoringCriticalAssetAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut creation_author_id: Option<i64> = None;
                let mut creation_date: Option<i64> = None;
                let mut creator: Option<crate::datadogV2::model::SecurityMonitoringUser> = None;
                let mut enabled: Option<bool> = None;
                let mut query: Option<String> = None;
                let mut rule_query: Option<String> = None;
                let mut severity: Option<
                    crate::datadogV2::model::SecurityMonitoringCriticalAssetSeverity,
                > = None;
                let mut tags: Option<Vec<String>> = None;
                let mut update_author_id: Option<i64> = None;
                let mut update_date: Option<i64> = None;
                let mut updater: Option<crate::datadogV2::model::SecurityMonitoringUser> = None;
                let mut version: Option<i32> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "creation_author_id" => {
                            if v.is_null() {
                                continue;
                            }
                            creation_author_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "creation_date" => {
                            if v.is_null() {
                                continue;
                            }
                            creation_date =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "creator" => {
                            if v.is_null() {
                                continue;
                            }
                            creator = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
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
                        "update_author_id" => {
                            if v.is_null() {
                                continue;
                            }
                            update_author_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "update_date" => {
                            if v.is_null() {
                                continue;
                            }
                            update_date =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updater" => {
                            if v.is_null() {
                                continue;
                            }
                            updater = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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

                let content = SecurityMonitoringCriticalAssetAttributes {
                    creation_author_id,
                    creation_date,
                    creator,
                    enabled,
                    query,
                    rule_query,
                    severity,
                    tags,
                    update_author_id,
                    update_date,
                    updater,
                    version,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityMonitoringCriticalAssetAttributesVisitor)
    }
}
