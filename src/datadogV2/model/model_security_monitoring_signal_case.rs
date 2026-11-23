// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A case linked to a security signal.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityMonitoringSignalCase {
    /// The UUID of the user assigned to the case.
    #[serde(rename = "assigneeId")]
    pub assignee_id: Option<String>,
    /// The name of the user assigned to the case.
    #[serde(rename = "assigneeName")]
    pub assignee_name: Option<String>,
    /// The unique ID of the case.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// The key of the case.
    #[serde(rename = "key")]
    pub key: Option<String>,
    /// The status of the case.
    #[serde(rename = "status")]
    pub status: Option<String>,
    /// The title of the case.
    #[serde(rename = "title")]
    pub title: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityMonitoringSignalCase {
    pub fn new() -> SecurityMonitoringSignalCase {
        SecurityMonitoringSignalCase {
            assignee_id: None,
            assignee_name: None,
            id: None,
            key: None,
            status: None,
            title: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn assignee_id(mut self, value: String) -> Self {
        self.assignee_id = Some(value);
        self
    }

    pub fn assignee_name(mut self, value: String) -> Self {
        self.assignee_name = Some(value);
        self
    }

    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }

    pub fn key(mut self, value: String) -> Self {
        self.key = Some(value);
        self
    }

    pub fn status(mut self, value: String) -> Self {
        self.status = Some(value);
        self
    }

    pub fn title(mut self, value: String) -> Self {
        self.title = Some(value);
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

impl Default for SecurityMonitoringSignalCase {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SecurityMonitoringSignalCase {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityMonitoringSignalCaseVisitor;
        impl<'a> Visitor<'a> for SecurityMonitoringSignalCaseVisitor {
            type Value = SecurityMonitoringSignalCase;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut assignee_id: Option<String> = None;
                let mut assignee_name: Option<String> = None;
                let mut id: Option<String> = None;
                let mut key: Option<String> = None;
                let mut status: Option<String> = None;
                let mut title: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "assigneeId" => {
                            if v.is_null() {
                                continue;
                            }
                            assignee_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "assigneeName" => {
                            if v.is_null() {
                                continue;
                            }
                            assignee_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            if v.is_null() {
                                continue;
                            }
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "key" => {
                            if v.is_null() {
                                continue;
                            }
                            key = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            if v.is_null() {
                                continue;
                            }
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "title" => {
                            if v.is_null() {
                                continue;
                            }
                            title = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SecurityMonitoringSignalCase {
                    assignee_id,
                    assignee_name,
                    id,
                    key,
                    status,
                    title,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityMonitoringSignalCaseVisitor)
    }
}
