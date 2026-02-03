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
pub struct SnapshotCreateRequestDataAttributes {
    #[serde(rename = "application_id")]
    pub application_id: String,
    #[serde(rename = "device_type")]
    pub device_type: String,
    #[serde(rename = "event_id")]
    pub event_id: String,
    #[serde(rename = "is_device_type_selected_by_user")]
    pub is_device_type_selected_by_user: bool,
    #[serde(rename = "session_id")]
    pub session_id: Option<String>,
    #[serde(rename = "snapshot_name")]
    pub snapshot_name: String,
    #[serde(rename = "start")]
    pub start: i64,
    #[serde(rename = "view_id")]
    pub view_id: Option<String>,
    #[serde(rename = "view_name")]
    pub view_name: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SnapshotCreateRequestDataAttributes {
    pub fn new(
        application_id: String,
        device_type: String,
        event_id: String,
        is_device_type_selected_by_user: bool,
        snapshot_name: String,
        start: i64,
        view_name: String,
    ) -> SnapshotCreateRequestDataAttributes {
        SnapshotCreateRequestDataAttributes {
            application_id,
            device_type,
            event_id,
            is_device_type_selected_by_user,
            session_id: None,
            snapshot_name,
            start,
            view_id: None,
            view_name,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn session_id(mut self, value: String) -> Self {
        self.session_id = Some(value);
        self
    }

    pub fn view_id(mut self, value: String) -> Self {
        self.view_id = Some(value);
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

impl<'de> Deserialize<'de> for SnapshotCreateRequestDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SnapshotCreateRequestDataAttributesVisitor;
        impl<'a> Visitor<'a> for SnapshotCreateRequestDataAttributesVisitor {
            type Value = SnapshotCreateRequestDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut application_id: Option<String> = None;
                let mut device_type: Option<String> = None;
                let mut event_id: Option<String> = None;
                let mut is_device_type_selected_by_user: Option<bool> = None;
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
                            application_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "device_type" => {
                            device_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "event_id" => {
                            event_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_device_type_selected_by_user" => {
                            is_device_type_selected_by_user =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "session_id" => {
                            if v.is_null() {
                                continue;
                            }
                            session_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "snapshot_name" => {
                            snapshot_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "start" => {
                            start = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "view_id" => {
                            if v.is_null() {
                                continue;
                            }
                            view_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "view_name" => {
                            view_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let application_id =
                    application_id.ok_or_else(|| M::Error::missing_field("application_id"))?;
                let device_type =
                    device_type.ok_or_else(|| M::Error::missing_field("device_type"))?;
                let event_id = event_id.ok_or_else(|| M::Error::missing_field("event_id"))?;
                let is_device_type_selected_by_user = is_device_type_selected_by_user
                    .ok_or_else(|| M::Error::missing_field("is_device_type_selected_by_user"))?;
                let snapshot_name =
                    snapshot_name.ok_or_else(|| M::Error::missing_field("snapshot_name"))?;
                let start = start.ok_or_else(|| M::Error::missing_field("start"))?;
                let view_name = view_name.ok_or_else(|| M::Error::missing_field("view_name"))?;

                let content = SnapshotCreateRequestDataAttributes {
                    application_id,
                    device_type,
                    event_id,
                    is_device_type_selected_by_user,
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

        deserializer.deserialize_any(SnapshotCreateRequestDataAttributesVisitor)
    }
}
