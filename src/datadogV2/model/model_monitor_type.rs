// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes from the monitor that triggered the event.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MonitorType {
    /// The POSIX timestamp of the monitor's creation in nanoseconds.
    #[serde(rename = "created_at")]
    pub created_at: Option<i64>,
    /// Monitor group status used when there is no `result_groups`.
    #[serde(rename = "group_status")]
    pub group_status: Option<i32>,
    /// Groups to which the monitor belongs.
    #[serde(rename = "groups")]
    pub groups: Option<Vec<String>>,
    /// The monitor ID.
    #[serde(rename = "id")]
    pub id: Option<i64>,
    /// The monitor message.
    #[serde(rename = "message")]
    pub message: Option<String>,
    /// The monitor's last-modified timestamp.
    #[serde(rename = "modified")]
    pub modified: Option<i64>,
    /// The monitor name.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// The query that triggers the alert.
    #[serde(rename = "query")]
    pub query: Option<String>,
    /// A list of tags attached to the monitor.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// The templated name of the monitor before resolving any template variables.
    #[serde(rename = "templated_name")]
    pub templated_name: Option<String>,
    /// The monitor type.
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MonitorType {
    pub fn new() -> MonitorType {
        MonitorType {
            created_at: None,
            group_status: None,
            groups: None,
            id: None,
            message: None,
            modified: None,
            name: None,
            query: None,
            tags: None,
            templated_name: None,
            type_: None,
            _unparsed: false,
        }
    }

    pub fn created_at(mut self, value: i64) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn group_status(mut self, value: i32) -> Self {
        self.group_status = Some(value);
        self
    }

    pub fn groups(mut self, value: Vec<String>) -> Self {
        self.groups = Some(value);
        self
    }

    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn message(mut self, value: String) -> Self {
        self.message = Some(value);
        self
    }

    pub fn modified(mut self, value: i64) -> Self {
        self.modified = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn query(mut self, value: String) -> Self {
        self.query = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn templated_name(mut self, value: String) -> Self {
        self.templated_name = Some(value);
        self
    }

    pub fn type_(mut self, value: String) -> Self {
        self.type_ = Some(value);
        self
    }
}

impl Default for MonitorType {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for MonitorType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MonitorTypeVisitor;
        impl<'a> Visitor<'a> for MonitorTypeVisitor {
            type Value = MonitorType;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_at: Option<i64> = None;
                let mut group_status: Option<i32> = None;
                let mut groups: Option<Vec<String>> = None;
                let mut id: Option<i64> = None;
                let mut message: Option<String> = None;
                let mut modified: Option<i64> = None;
                let mut name: Option<String> = None;
                let mut query: Option<String> = None;
                let mut tags: Option<Vec<String>> = None;
                let mut templated_name: Option<String> = None;
                let mut type_: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "created_at" => {
                            if v.is_null() {
                                continue;
                            }
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "group_status" => {
                            if v.is_null() {
                                continue;
                            }
                            group_status =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "groups" => {
                            if v.is_null() {
                                continue;
                            }
                            groups = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            if v.is_null() {
                                continue;
                            }
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "message" => {
                            if v.is_null() {
                                continue;
                            }
                            message = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified" => {
                            if v.is_null() {
                                continue;
                            }
                            modified = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "query" => {
                            if v.is_null() {
                                continue;
                            }
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "templated_name" => {
                            if v.is_null() {
                                continue;
                            }
                            templated_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = MonitorType {
                    created_at,
                    group_status,
                    groups,
                    id,
                    message,
                    modified,
                    name,
                    query,
                    tags,
                    templated_name,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MonitorTypeVisitor)
    }
}
