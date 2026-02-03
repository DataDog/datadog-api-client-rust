// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SnapshotDataAttributes {
    #[serde(rename = "application_id")]
    pub application_id: Option<String>,
    #[serde(rename = "created_at")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "created_by")]
    pub created_by: Option<String>,
    #[serde(rename = "created_by_handle")]
    pub created_by_handle: Option<String>,
    #[serde(rename = "created_by_user_id")]
    pub created_by_user_id: Option<i64>,
    #[serde(rename = "device_type")]
    pub device_type: Option<String>,
    #[serde(rename = "event_id")]
    pub event_id: Option<String>,
    #[serde(rename = "is_device_type_selected_by_user")]
    pub is_device_type_selected_by_user: Option<bool>,
    #[serde(rename = "modified_at")]
    pub modified_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "org_id")]
    pub org_id: Option<i64>,
    #[serde(rename = "session_id")]
    pub session_id: Option<String>,
    #[serde(rename = "snapshot_name")]
    pub snapshot_name: Option<String>,
    #[serde(rename = "start")]
    pub start: Option<i64>,
    #[serde(rename = "view_id")]
    pub view_id: Option<String>,
    #[serde(rename = "view_name")]
    pub view_name: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SnapshotDataAttributes {
    pub fn new() -> SnapshotDataAttributes {
        SnapshotDataAttributes {
            application_id: None,
            created_at: None,
            created_by: None,
            created_by_handle: None,
            created_by_user_id: None,
            device_type: None,
            event_id: None,
            is_device_type_selected_by_user: None,
            modified_at: None,
            org_id: None,
            session_id: None,
            snapshot_name: None,
            start: None,
            view_id: None,
            view_name: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn application_id(mut self, value: String) -> Self {
        self.application_id = Some(value);
        self
    }

    pub fn created_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn created_by(mut self, value: String) -> Self {
        self.created_by = Some(value);
        self
    }

    pub fn created_by_handle(mut self, value: String) -> Self {
        self.created_by_handle = Some(value);
        self
    }

    pub fn created_by_user_id(mut self, value: i64) -> Self {
        self.created_by_user_id = Some(value);
        self
    }

    pub fn device_type(mut self, value: String) -> Self {
        self.device_type = Some(value);
        self
    }

    pub fn event_id(mut self, value: String) -> Self {
        self.event_id = Some(value);
        self
    }

    pub fn is_device_type_selected_by_user(mut self, value: bool) -> Self {
        self.is_device_type_selected_by_user = Some(value);
        self
    }

    pub fn modified_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.modified_at = Some(value);
        self
    }

    pub fn org_id(mut self, value: i64) -> Self {
        self.org_id = Some(value);
        self
    }

    pub fn session_id(mut self, value: String) -> Self {
        self.session_id = Some(value);
        self
    }

    pub fn snapshot_name(mut self, value: String) -> Self {
        self.snapshot_name = Some(value);
        self
    }

    pub fn start(mut self, value: i64) -> Self {
        self.start = Some(value);
        self
    }

    pub fn view_id(mut self, value: String) -> Self {
        self.view_id = Some(value);
        self
    }

    pub fn view_name(mut self, value: String) -> Self {
        self.view_name = Some(value);
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

impl Default for SnapshotDataAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SnapshotDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SnapshotDataAttributesVisitor;
        impl<'a> Visitor<'a> for SnapshotDataAttributesVisitor {
            type Value = SnapshotDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut application_id: Option<String> = None;
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut created_by: Option<String> = None;
                let mut created_by_handle: Option<String> = None;
                let mut created_by_user_id: Option<i64> = None;
                let mut device_type: Option<String> = None;
                let mut event_id: Option<String> = None;
                let mut is_device_type_selected_by_user: Option<bool> = None;
                let mut modified_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut org_id: Option<i64> = None;
                let mut session_id: Option<String> = None;
                let mut snapshot_name: Option<String> = None;
                let mut start: Option<i64> = None;
                let mut view_id: Option<String> = None;
                let mut view_name: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "application_id" => {
                            if v.is_null() {
                                continue;
                            }
                            application_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_at" => {
                            if v.is_null() {
                                continue;
                            }
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_by" => {
                            if v.is_null() {
                                continue;
                            }
                            created_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_by_handle" => {
                            if v.is_null() {
                                continue;
                            }
                            created_by_handle =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_by_user_id" => {
                            if v.is_null() {
                                continue;
                            }
                            created_by_user_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "device_type" => {
                            if v.is_null() {
                                continue;
                            }
                            device_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "event_id" => {
                            if v.is_null() {
                                continue;
                            }
                            event_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_device_type_selected_by_user" => {
                            if v.is_null() {
                                continue;
                            }
                            is_device_type_selected_by_user =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified_at" => {
                            if v.is_null() {
                                continue;
                            }
                            modified_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "org_id" => {
                            if v.is_null() {
                                continue;
                            }
                            org_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "session_id" => {
                            if v.is_null() {
                                continue;
                            }
                            session_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "snapshot_name" => {
                            if v.is_null() {
                                continue;
                            }
                            snapshot_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "start" => {
                            if v.is_null() {
                                continue;
                            }
                            start = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "view_id" => {
                            if v.is_null() {
                                continue;
                            }
                            view_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "view_name" => {
                            if v.is_null() {
                                continue;
                            }
                            view_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SnapshotDataAttributes {
                    application_id,
                    created_at,
                    created_by,
                    created_by_handle,
                    created_by_user_id,
                    device_type,
                    event_id,
                    is_device_type_selected_by_user,
                    modified_at,
                    org_id,
                    session_id,
                    snapshot_name,
                    start,
                    view_id,
                    view_name,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SnapshotDataAttributesVisitor)
    }
}
