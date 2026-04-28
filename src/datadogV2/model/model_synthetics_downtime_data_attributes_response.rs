// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a Synthetics downtime response object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsDowntimeDataAttributesResponse {
    /// The timestamp when the downtime was created.
    #[serde(rename = "createdAt")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// The UUID of the user who created the downtime.
    #[serde(rename = "createdBy")]
    pub created_by: String,
    /// The display name of the user who created the downtime.
    #[serde(rename = "createdByName")]
    pub created_by_name: String,
    /// The description of the downtime.
    #[serde(rename = "description")]
    pub description: String,
    /// Whether the downtime is enabled.
    #[serde(rename = "isEnabled")]
    pub is_enabled: bool,
    /// The name of the downtime.
    #[serde(rename = "name")]
    pub name: String,
    /// List of tags associated with a Synthetics downtime.
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
    /// List of Synthetics test public IDs associated with a downtime.
    #[serde(rename = "testIds")]
    pub test_ids: Vec<String>,
    /// List of time slots in a Synthetics downtime response.
    #[serde(rename = "timeSlots")]
    pub time_slots: Vec<crate::datadogV2::model::SyntheticsDowntimeTimeSlotResponse>,
    /// The timestamp when the downtime was last updated.
    #[serde(rename = "updatedAt")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    /// The UUID of the user who last updated the downtime.
    #[serde(rename = "updatedBy")]
    pub updated_by: String,
    /// The display name of the user who last updated the downtime.
    #[serde(rename = "updatedByName")]
    pub updated_by_name: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsDowntimeDataAttributesResponse {
    pub fn new(
        created_at: chrono::DateTime<chrono::Utc>,
        created_by: String,
        created_by_name: String,
        description: String,
        is_enabled: bool,
        name: String,
        tags: Vec<String>,
        test_ids: Vec<String>,
        time_slots: Vec<crate::datadogV2::model::SyntheticsDowntimeTimeSlotResponse>,
        updated_at: chrono::DateTime<chrono::Utc>,
        updated_by: String,
        updated_by_name: String,
    ) -> SyntheticsDowntimeDataAttributesResponse {
        SyntheticsDowntimeDataAttributesResponse {
            created_at,
            created_by,
            created_by_name,
            description,
            is_enabled,
            name,
            tags,
            test_ids,
            time_slots,
            updated_at,
            updated_by,
            updated_by_name,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for SyntheticsDowntimeDataAttributesResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsDowntimeDataAttributesResponseVisitor;
        impl<'a> Visitor<'a> for SyntheticsDowntimeDataAttributesResponseVisitor {
            type Value = SyntheticsDowntimeDataAttributesResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut created_by: Option<String> = None;
                let mut created_by_name: Option<String> = None;
                let mut description: Option<String> = None;
                let mut is_enabled: Option<bool> = None;
                let mut name: Option<String> = None;
                let mut tags: Option<Vec<String>> = None;
                let mut test_ids: Option<Vec<String>> = None;
                let mut time_slots: Option<
                    Vec<crate::datadogV2::model::SyntheticsDowntimeTimeSlotResponse>,
                > = None;
                let mut updated_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut updated_by: Option<String> = None;
                let mut updated_by_name: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "createdAt" => {
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "createdBy" => {
                            created_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "createdByName" => {
                            created_by_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
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
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "testIds" => {
                            test_ids = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "timeSlots" => {
                            time_slots = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updatedAt" => {
                            updated_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updatedBy" => {
                            updated_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updatedByName" => {
                            updated_by_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let created_by = created_by.ok_or_else(|| M::Error::missing_field("created_by"))?;
                let created_by_name =
                    created_by_name.ok_or_else(|| M::Error::missing_field("created_by_name"))?;
                let description =
                    description.ok_or_else(|| M::Error::missing_field("description"))?;
                let is_enabled = is_enabled.ok_or_else(|| M::Error::missing_field("is_enabled"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let tags = tags.ok_or_else(|| M::Error::missing_field("tags"))?;
                let test_ids = test_ids.ok_or_else(|| M::Error::missing_field("test_ids"))?;
                let time_slots = time_slots.ok_or_else(|| M::Error::missing_field("time_slots"))?;
                let updated_at = updated_at.ok_or_else(|| M::Error::missing_field("updated_at"))?;
                let updated_by = updated_by.ok_or_else(|| M::Error::missing_field("updated_by"))?;
                let updated_by_name =
                    updated_by_name.ok_or_else(|| M::Error::missing_field("updated_by_name"))?;

                let content = SyntheticsDowntimeDataAttributesResponse {
                    created_at,
                    created_by,
                    created_by_name,
                    description,
                    is_enabled,
                    name,
                    tags,
                    test_ids,
                    time_slots,
                    updated_at,
                    updated_by,
                    updated_by_name,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsDowntimeDataAttributesResponseVisitor)
    }
}
