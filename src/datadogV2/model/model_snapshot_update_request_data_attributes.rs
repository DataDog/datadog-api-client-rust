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
pub struct SnapshotUpdateRequestDataAttributes {
    #[serde(rename = "event_id")]
    pub event_id: String,
    #[serde(rename = "is_device_type_selected_by_user")]
    pub is_device_type_selected_by_user: bool,
    #[serde(rename = "session_id")]
    pub session_id: Option<String>,
    #[serde(rename = "start")]
    pub start: i64,
    #[serde(rename = "view_id")]
    pub view_id: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SnapshotUpdateRequestDataAttributes {
    pub fn new(
        event_id: String,
        is_device_type_selected_by_user: bool,
        start: i64,
    ) -> SnapshotUpdateRequestDataAttributes {
        SnapshotUpdateRequestDataAttributes {
            event_id,
            is_device_type_selected_by_user,
            session_id: None,
            start,
            view_id: None,
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

impl<'de> Deserialize<'de> for SnapshotUpdateRequestDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SnapshotUpdateRequestDataAttributesVisitor;
        impl<'a> Visitor<'a> for SnapshotUpdateRequestDataAttributesVisitor {
            type Value = SnapshotUpdateRequestDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut event_id: Option<String> = None;
                let mut is_device_type_selected_by_user: Option<bool> = None;
                let mut session_id: Option<String> = None;
                let mut start: Option<i64> = None;
                let mut view_id: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
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
                        "start" => {
                            start = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "view_id" => {
                            if v.is_null() {
                                continue;
                            }
                            view_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let event_id = event_id.ok_or_else(|| M::Error::missing_field("event_id"))?;
                let is_device_type_selected_by_user = is_device_type_selected_by_user
                    .ok_or_else(|| M::Error::missing_field("is_device_type_selected_by_user"))?;
                let start = start.ok_or_else(|| M::Error::missing_field("start"))?;

                let content = SnapshotUpdateRequestDataAttributes {
                    event_id,
                    is_device_type_selected_by_user,
                    session_id,
                    start,
                    view_id,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SnapshotUpdateRequestDataAttributesVisitor)
    }
}
