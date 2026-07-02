// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for snapshot creation.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CreateSnapshotDataAttributesRequest {
    /// Additional configuration options for snapshot creation.
    #[serde(rename = "additional_config")]
    pub additional_config: Option<crate::datadogV2::model::CreateSnapshotAdditionalConfig>,
    /// End of the time window for the snapshot, in milliseconds since Unix epoch.
    #[serde(rename = "end")]
    pub end: i64,
    /// The height of the rendered snapshot in pixels.
    #[serde(rename = "height")]
    pub height: Option<i64>,
    /// Whether the snapshot requires authentication to view. Authenticated snapshots are scoped to the creating organization.
    #[serde(rename = "is_authenticated")]
    pub is_authenticated: Option<bool>,
    /// Start of the time window for the snapshot, in milliseconds since Unix epoch.
    #[serde(rename = "start")]
    pub start: i64,
    /// The time-to-live for the snapshot. This value corresponds to storage lifecycle policies that automatically delete the snapshot after the specified period.
    #[serde(rename = "ttl")]
    pub ttl: Option<crate::datadogV2::model::CreateSnapshotTTL>,
    /// The widget definition to render as a snapshot. Must include a valid `type` field and non-empty `requests` array.
    #[serde(rename = "widget_definition")]
    pub widget_definition: std::collections::BTreeMap<String, serde_json::Value>,
    /// The width of the rendered snapshot in pixels.
    #[serde(rename = "width")]
    pub width: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CreateSnapshotDataAttributesRequest {
    pub fn new(
        end: i64,
        start: i64,
        widget_definition: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> CreateSnapshotDataAttributesRequest {
        CreateSnapshotDataAttributesRequest {
            additional_config: None,
            end,
            height: None,
            is_authenticated: None,
            start,
            ttl: None,
            widget_definition,
            width: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_config(
        mut self,
        value: crate::datadogV2::model::CreateSnapshotAdditionalConfig,
    ) -> Self {
        self.additional_config = Some(value);
        self
    }

    pub fn height(mut self, value: i64) -> Self {
        self.height = Some(value);
        self
    }

    pub fn is_authenticated(mut self, value: bool) -> Self {
        self.is_authenticated = Some(value);
        self
    }

    pub fn ttl(mut self, value: crate::datadogV2::model::CreateSnapshotTTL) -> Self {
        self.ttl = Some(value);
        self
    }

    pub fn width(mut self, value: i64) -> Self {
        self.width = Some(value);
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

impl<'de> Deserialize<'de> for CreateSnapshotDataAttributesRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CreateSnapshotDataAttributesRequestVisitor;
        impl<'a> Visitor<'a> for CreateSnapshotDataAttributesRequestVisitor {
            type Value = CreateSnapshotDataAttributesRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut additional_config: Option<
                    crate::datadogV2::model::CreateSnapshotAdditionalConfig,
                > = None;
                let mut end: Option<i64> = None;
                let mut height: Option<i64> = None;
                let mut is_authenticated: Option<bool> = None;
                let mut start: Option<i64> = None;
                let mut ttl: Option<crate::datadogV2::model::CreateSnapshotTTL> = None;
                let mut widget_definition: Option<
                    std::collections::BTreeMap<String, serde_json::Value>,
                > = None;
                let mut width: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "additional_config" => {
                            if v.is_null() {
                                continue;
                            }
                            additional_config =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "end" => {
                            end = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "height" => {
                            if v.is_null() {
                                continue;
                            }
                            height = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_authenticated" => {
                            if v.is_null() {
                                continue;
                            }
                            is_authenticated =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "start" => {
                            start = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ttl" => {
                            if v.is_null() {
                                continue;
                            }
                            ttl = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _ttl) = ttl {
                                match _ttl {
                                    crate::datadogV2::model::CreateSnapshotTTL::UnparsedObject(
                                        _ttl,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "widget_definition" => {
                            widget_definition =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "width" => {
                            if v.is_null() {
                                continue;
                            }
                            width = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let end = end.ok_or_else(|| M::Error::missing_field("end"))?;
                let start = start.ok_or_else(|| M::Error::missing_field("start"))?;
                let widget_definition = widget_definition
                    .ok_or_else(|| M::Error::missing_field("widget_definition"))?;

                let content = CreateSnapshotDataAttributesRequest {
                    additional_config,
                    end,
                    height,
                    is_authenticated,
                    start,
                    ttl,
                    widget_definition,
                    width,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CreateSnapshotDataAttributesRequestVisitor)
    }
}
