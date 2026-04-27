// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for creating or updating a Synthetics downtime.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsDowntimeDataAttributesRequest {
    /// An optional description of the downtime.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Whether the downtime is enabled.
    #[serde(rename = "isEnabled")]
    pub is_enabled: bool,
    /// The name of the downtime.
    #[serde(rename = "name")]
    pub name: String,
    /// List of tags associated with a Synthetics downtime.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// List of Synthetics test public IDs associated with a downtime.
    #[serde(rename = "testIds")]
    pub test_ids: Vec<String>,
    /// List of time slots for a Synthetics downtime create or update request.
    #[serde(rename = "timeSlots")]
    pub time_slots: Vec<crate::datadogV2::model::SyntheticsDowntimeTimeSlotRequest>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsDowntimeDataAttributesRequest {
    pub fn new(
        is_enabled: bool,
        name: String,
        test_ids: Vec<String>,
        time_slots: Vec<crate::datadogV2::model::SyntheticsDowntimeTimeSlotRequest>,
    ) -> SyntheticsDowntimeDataAttributesRequest {
        SyntheticsDowntimeDataAttributesRequest {
            description: None,
            is_enabled,
            name,
            tags: None,
            test_ids,
            time_slots,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
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

impl<'de> Deserialize<'de> for SyntheticsDowntimeDataAttributesRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsDowntimeDataAttributesRequestVisitor;
        impl<'a> Visitor<'a> for SyntheticsDowntimeDataAttributesRequestVisitor {
            type Value = SyntheticsDowntimeDataAttributesRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut description: Option<String> = None;
                let mut is_enabled: Option<bool> = None;
                let mut name: Option<String> = None;
                let mut tags: Option<Vec<String>> = None;
                let mut test_ids: Option<Vec<String>> = None;
                let mut time_slots: Option<
                    Vec<crate::datadogV2::model::SyntheticsDowntimeTimeSlotRequest>,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "isEnabled" => {
                            is_enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "testIds" => {
                            test_ids = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "timeSlots" => {
                            time_slots = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let is_enabled = is_enabled.ok_or_else(|| M::Error::missing_field("is_enabled"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let test_ids = test_ids.ok_or_else(|| M::Error::missing_field("test_ids"))?;
                let time_slots = time_slots.ok_or_else(|| M::Error::missing_field("time_slots"))?;

                let content = SyntheticsDowntimeDataAttributesRequest {
                    description,
                    is_enabled,
                    name,
                    tags,
                    test_ids,
                    time_slots,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsDowntimeDataAttributesRequestVisitor)
    }
}
