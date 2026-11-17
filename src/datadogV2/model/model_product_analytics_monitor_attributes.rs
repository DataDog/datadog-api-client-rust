// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a Product Analytics monitor.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ProductAnalyticsMonitorAttributes {
    /// Time the monitor was created.
    #[serde(rename = "created_time")]
    pub created_time: Option<chrono::DateTime<chrono::Utc>>,
    /// The user who created the monitor.
    #[serde(rename = "creator")]
    pub creator: Option<crate::datadogV2::model::ProductAnalyticsMonitorCreator>,
    /// Notification message sent when the monitor alerts.
    #[serde(rename = "message")]
    pub message: String,
    /// Time the monitor was last modified.
    #[serde(rename = "modified_time")]
    pub modified_time: Option<chrono::DateTime<chrono::Utc>>,
    /// Name of the monitor.
    #[serde(rename = "name")]
    pub name: String,
    /// Options of a Product Analytics monitor.
    #[serde(rename = "options")]
    pub options: crate::datadogV2::model::ProductAnalyticsMonitorOptions,
    /// ID of the organization that owns the monitor.
    #[serde(rename = "org_id")]
    pub org_id: i64,
    /// Priority of the monitor, from 1 (highest) to 5 (lowest).
    #[serde(rename = "priority")]
    pub priority: Option<i32>,
    /// Monitor query.
    #[serde(rename = "query")]
    pub query: String,
    /// Current alert status of the monitor. `No Data` is returned when the monitor has not been evaluated yet.
    #[serde(rename = "status")]
    pub status: crate::datadogV2::model::ProductAnalyticsMonitorStatus,
    /// Monitor tags, as `key:value` strings.
    #[serde(rename = "tags", default, with = "::serde_with::rust::double_option")]
    pub tags: Option<Option<Vec<String>>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ProductAnalyticsMonitorAttributes {
    pub fn new(
        message: String,
        name: String,
        options: crate::datadogV2::model::ProductAnalyticsMonitorOptions,
        org_id: i64,
        query: String,
        status: crate::datadogV2::model::ProductAnalyticsMonitorStatus,
    ) -> ProductAnalyticsMonitorAttributes {
        ProductAnalyticsMonitorAttributes {
            created_time: None,
            creator: None,
            message,
            modified_time: None,
            name,
            options,
            org_id,
            priority: None,
            query,
            status,
            tags: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn created_time(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.created_time = Some(value);
        self
    }

    pub fn creator(
        mut self,
        value: crate::datadogV2::model::ProductAnalyticsMonitorCreator,
    ) -> Self {
        self.creator = Some(value);
        self
    }

    pub fn modified_time(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.modified_time = Some(value);
        self
    }

    pub fn priority(mut self, value: i32) -> Self {
        self.priority = Some(value);
        self
    }

    pub fn tags(mut self, value: Option<Vec<String>>) -> Self {
        self.tags = Some(value);
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

impl<'de> Deserialize<'de> for ProductAnalyticsMonitorAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProductAnalyticsMonitorAttributesVisitor;
        impl<'a> Visitor<'a> for ProductAnalyticsMonitorAttributesVisitor {
            type Value = ProductAnalyticsMonitorAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_time: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut creator: Option<crate::datadogV2::model::ProductAnalyticsMonitorCreator> =
                    None;
                let mut message: Option<String> = None;
                let mut modified_time: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut name: Option<String> = None;
                let mut options: Option<crate::datadogV2::model::ProductAnalyticsMonitorOptions> =
                    None;
                let mut org_id: Option<i64> = None;
                let mut priority: Option<i32> = None;
                let mut query: Option<String> = None;
                let mut status: Option<crate::datadogV2::model::ProductAnalyticsMonitorStatus> =
                    None;
                let mut tags: Option<Option<Vec<String>>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "created_time" => {
                            if v.is_null() {
                                continue;
                            }
                            created_time =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "creator" => {
                            if v.is_null() {
                                continue;
                            }
                            creator = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "message" => {
                            message = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified_time" => {
                            if v.is_null() {
                                continue;
                            }
                            modified_time =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "options" => {
                            options = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "org_id" => {
                            org_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "priority" => {
                            if v.is_null() {
                                continue;
                            }
                            priority = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "query" => {
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _status) = status {
                                match _status {
                                    crate::datadogV2::model::ProductAnalyticsMonitorStatus::UnparsedObject(_status) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "tags" => {
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let message = message.ok_or_else(|| M::Error::missing_field("message"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let options = options.ok_or_else(|| M::Error::missing_field("options"))?;
                let org_id = org_id.ok_or_else(|| M::Error::missing_field("org_id"))?;
                let query = query.ok_or_else(|| M::Error::missing_field("query"))?;
                let status = status.ok_or_else(|| M::Error::missing_field("status"))?;

                let content = ProductAnalyticsMonitorAttributes {
                    created_time,
                    creator,
                    message,
                    modified_time,
                    name,
                    options,
                    org_id,
                    priority,
                    query,
                    status,
                    tags,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ProductAnalyticsMonitorAttributesVisitor)
    }
}
