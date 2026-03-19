// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a deleted segment response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RumSegmentDeleteAttributes {
    /// The timestamp when the segment was disabled in RFC 3339 format.
    #[serde(rename = "disabled_at")]
    pub disabled_at: chrono::DateTime<chrono::Utc>,
    /// A user who performed an action on a segment.
    #[serde(rename = "disabled_by")]
    pub disabled_by: crate::datadogV2::model::RumSegmentUser,
    /// The name of the deleted segment.
    #[serde(rename = "name")]
    pub name: String,
    /// The unique identifier of the deleted segment.
    #[serde(rename = "uuid")]
    pub uuid: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RumSegmentDeleteAttributes {
    pub fn new(
        disabled_at: chrono::DateTime<chrono::Utc>,
        disabled_by: crate::datadogV2::model::RumSegmentUser,
        name: String,
        uuid: String,
    ) -> RumSegmentDeleteAttributes {
        RumSegmentDeleteAttributes {
            disabled_at,
            disabled_by,
            name,
            uuid,
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

impl<'de> Deserialize<'de> for RumSegmentDeleteAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RumSegmentDeleteAttributesVisitor;
        impl<'a> Visitor<'a> for RumSegmentDeleteAttributesVisitor {
            type Value = RumSegmentDeleteAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut disabled_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut disabled_by: Option<crate::datadogV2::model::RumSegmentUser> = None;
                let mut name: Option<String> = None;
                let mut uuid: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "disabled_at" => {
                            disabled_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "disabled_by" => {
                            disabled_by =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "uuid" => {
                            uuid = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let disabled_at =
                    disabled_at.ok_or_else(|| M::Error::missing_field("disabled_at"))?;
                let disabled_by =
                    disabled_by.ok_or_else(|| M::Error::missing_field("disabled_by"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let uuid = uuid.ok_or_else(|| M::Error::missing_field("uuid"))?;

                let content = RumSegmentDeleteAttributes {
                    disabled_at,
                    disabled_by,
                    name,
                    uuid,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RumSegmentDeleteAttributesVisitor)
    }
}
